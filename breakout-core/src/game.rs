use ball::Ball;
use block::Block;
use math;
use paddle::Paddle;
use rand;

use object::GameObject;

pub struct BreakoutBuilder {
    dt: Option<f32>,
    ball: Option<Ball>,
    paddle: Option<Paddle>,
    blocks: Vec<Block>,
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

    pub fn build(self) -> Breakout {
        let BreakoutBuilder {
            dt,
            ball,
            paddle,
            blocks,
        } = self;

        Breakout {
            dt: dt.unwrap(),
            ball: ball.unwrap(),
            paddle: paddle.unwrap(),
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
    blocks: Vec<Block>,
}

impl Breakout {
    pub fn new_game(dt: f32) -> Self {
        let ball = {
            let x_speed: f32 = rand::random::<f32>() - 0.5;
            let y_speed = (1. - x_speed * x_speed).sqrt();
            Ball::new(0.02, (0.5, 0.5), (x_speed * 0.1, y_speed * -0.1))
        };

        let paddle = Paddle::new((0.1, 0.04), (0.5, 0.05));

        Self {
            dt,
            ball,
            paddle,
            blocks: vec![],
        }
    }

    pub fn dt(&self) -> f32 {
        self.dt
    }

    pub fn paddle(&self) -> &Paddle {
        &self.paddle
    }

    pub fn ball(&self) -> &Ball {
        &self.ball
    }

    fn tick_positions(&mut self) {
        math::tick_position(&mut self.ball, self.dt);
        math::tick_position(&mut self.paddle, self.dt);
    }

    fn resolve_collisions(&mut self) {
        // Check for collisions and make corrections

        if math::objects_are_close(&self.ball, &self.paddle) {
            let (ball_x, ball_y) = self.ball.location();
            let ball_r = self.ball.radius();

            let ((left_paddle, top_paddle), (right_paddle, bottom_paddle)) =
                self.paddle.boundaries();

            // Easy, but non-exhaustive check
            if ball_x >= left_paddle && ball_x <= right_paddle {
                if ball_y - ball_r <= top_paddle && ball_y + ball_r >= bottom_paddle {
                    // Move ball back above paddle by amount it "dipped" into it and reverse y
                    // component of velocity. This needs a more exhaustive and correct
                    // implementation.
                    let dip = top_paddle - (ball_y - ball_r);
                    self.ball.set_location((ball_x, top_paddle + dip + ball_r));

                    let (ball_vx, ball_vy) = self.ball.velocity();
                    self.ball.set_velocity((ball_vx, -ball_vy));
                }
            }
        }
    }

    pub fn tick(&mut self) {
        self.tick_positions();
        self.resolve_collisions();
    }
}
