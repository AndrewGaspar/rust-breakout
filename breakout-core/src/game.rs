use ball::Ball;
use block::Block;
use math;
use paddle::Paddle;

use object::GameObject;

pub struct BreakoutBuilder {
    dt: Option<f32>,
    ball: Option<Ball>,
    paddle: Option<Paddle>,
    blocks: Vec<Option<Block>>,
}

impl BreakoutBuilder {
    pub fn new() -> Self {
        Self {
            dt: None,
            ball: None,
            paddle: None,
            blocks: vec![],
        }
    }

    pub fn dt(mut self, dt: f32) -> Self {
        self.dt = Some(dt);
        self
    }

    pub fn ball(mut self, ball: Ball) -> Self {
        self.ball = Some(ball);
        self
    }

    pub fn paddle(mut self, paddle: Paddle) -> Self {
        self.paddle = Some(paddle);
        self
    }

    pub fn add_block(mut self, block: Block) -> Self {
        self.blocks.push(Some(block));
        self
    }

    pub fn add_blocks<I: Iterator<Item = Block>>(mut self, blocks: I) -> Self {
        self.blocks.extend(blocks.map(Some));
        self
    }

    pub fn build(self) -> Breakout {
        let BreakoutBuilder {
            dt,
            ball,
            paddle,
            blocks,
        } = self;

        Breakout {
            dt: dt.expect("User did not call BreakoutBuilder::dt(f32)"),
            ball: ball.expect("User did not call BreakoutBuilder::ball(Ball)"),
            paddle: paddle.expect("User did not call BreakoutBuilder::paddle(Paddle)"),
            blocks,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Breakout {
    /// This is the time delta the simulation runs at. Each tick occurs exactly once every dt.
    dt: f32,

    /// Holds data for the game Ball
    ball: Ball,

    /// The user's paddle.
    paddle: Paddle,

    /// The blocks in the game space. Sorted from closest to furthest.
    blocks: Vec<Option<Block>>,
}

impl Breakout {
    pub fn level_1(dt: f32) -> Self {
        BreakoutBuilder::new()
            .dt(dt)
            .ball(Ball::new(0.015, [0.5, 0.7], [0., -0.5]))
            .paddle(Paddle::new([0.15, 0.02], [0.425, 0.065]))
            .add_blocks(
                (0..4_i32).map(|i| Block::new([0.10, 0.05], [0.2 * (i + 1) as f32 - 0.05, 0.725])),
            )
            .build()
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn paddle(&self) -> &Paddle {
        &self.paddle
    }

    pub fn paddle_mut(&mut self) -> &mut Paddle {
        &mut self.paddle
    }

    pub fn ball(&self) -> &Ball {
        &self.ball
    }

    pub fn blocks(&self) -> &[Option<Block>] {
        &self.blocks[..]
    }

    fn tick_positions(&mut self) {
        math::tick_position(&mut self.ball, self.dt);
        math::tick_position(&mut self.paddle, self.dt);
    }

    fn resolve_ball_collisions(&mut self) {
        let [ball_x, ball_y] = self.ball.location();
        let ball_r = self.ball.radius();

        // Check for collisions and make corrections
        if math::objects_are_close(&self.ball, &self.paddle) {
            let ([paddle_left, paddle_top], [paddle_right, paddle_bottom]) =
                self.paddle.boundaries();
            let paddle_mid = (paddle_left + paddle_right) * 0.5;
            let [paddle_len, _] = self.paddle.dimensions();

            // Easy, but non-exhaustive check
            if ball_x >= paddle_left && ball_x <= paddle_right {
                if ball_y - ball_r <= paddle_top && ball_y + ball_r >= paddle_bottom {
                    // Move ball back above paddle by amount it "dipped" into it and reverse y
                    // component of velocity. This needs a more exhaustive and correct
                    // implementation.
                    let dip = paddle_top - (ball_y - ball_r);
                    self.ball.set_location([ball_x, paddle_top + dip + ball_r]);

                    let [ball_vx, ball_vy] = self.ball.velocity();

                    let speed = (ball_vx * ball_vx + ball_vy * ball_vy).sqrt() * 1.05;

                    let offset_from_paddle_center = ball_x - paddle_mid;

                    // will be negative when on the left side of paddle.
                    let percent_from_paddle_center = offset_from_paddle_center / (paddle_len * 0.5);

                    // limit the maximum x component
                    let percent_speed_in_x_direction = percent_from_paddle_center * 0.8;

                    let new_ball_vx = percent_speed_in_x_direction * speed;
                    let new_ball_vy = (speed * speed - new_ball_vx * new_ball_vx).sqrt();

                    self.ball.set_velocity([new_ball_vx, new_ball_vy]);
                }
            }
        }

        let [ball_x, ball_y] = self.ball.location();

        // Check if hits top of screen
        if ball_y + ball_r >= 1.0 {
            let passed = ball_y + ball_r - 1.0;
            self.ball.set_location([ball_x, 1.0 - passed - ball_r]);

            let [ball_vx, ball_vy] = self.ball.velocity();
            self.ball.set_velocity([ball_vx, -ball_vy]);
        }

        let [ball_x, ball_y] = self.ball.location();

        // left side of screen
        if ball_x - ball_r <= 0.0 {
            self.ball.set_location([ball_r - ball_x, ball_y]);

            let [ball_vx, ball_vy] = self.ball.velocity();
            self.ball.set_velocity([-ball_vx, ball_vy]);
        }

        let [ball_x, ball_y] = self.ball.location();

        // right side of screen
        if ball_x + ball_r >= 1.0 {
            let passed = ball_x + ball_r - 1.0;
            self.ball.set_location([1.0 - passed - ball_r, ball_y]);

            let [ball_vx, ball_vy] = self.ball.velocity();
            self.ball.set_velocity([-ball_vx, ball_vy]);
        }
    }

    fn resolve_paddle_collisions(&mut self) {
        let ([paddle_left, _], [paddle_right, paddle_bottom]) = self.paddle.boundaries();
        let [paddle_len, _] = self.paddle.dimensions();

        if paddle_right >= 1.0 {
            self.paddle.set_location([1.0 - paddle_len, paddle_bottom]);
            self.paddle.set_velocity([0., 0.]);
        }

        if paddle_left <= 0.0 {
            self.paddle.set_location([0.0, paddle_bottom]);
            self.paddle.set_velocity([0., 0.]);
        }
    }

    fn resolve_ball_block_collisions(&mut self) {
        for block in &mut self.blocks {
            if block.is_some() {
                // Check for collisions and make corrections
                if math::objects_are_close(&self.ball, block.as_ref().unwrap()) {
                    *block = None;
                }
            }
        }
    }

    fn resolve_collisions(&mut self) {
        self.resolve_ball_collisions();
        self.resolve_paddle_collisions();
        self.resolve_ball_block_collisions();
    }

    pub fn tick(&mut self) {
        self.tick_positions();
        self.resolve_collisions();
    }
}
