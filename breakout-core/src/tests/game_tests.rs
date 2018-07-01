use super::super::{Ball, BreakoutBuilder, GameObject, Paddle};

#[test]
fn basic() {
    // Creates a Breakout game where the ball is directly above the paddle and traveling
    // perpendicular towards it at 0.1 units per second. Under our simple assumption that momentum
    // is completely conserved and the paddle is of infinite mass, we expect the ball to bounce
    // back and travel away from the paddle at a rate of 0.1 units per second.
    let mut game = BreakoutBuilder::new()
        .dt(1. / 120.)
        .ball(Ball::new(0.02, (0.5, 0.24), (0., -0.1)))
        .paddle(Paddle::new((0.1, 0.04), (0.5, 0.20)))
        .build();

    // Run 120 frames - that should be 1 second, which should result in the ball being at
    // (0.5, 0.34)
    for _ in 0..120 {
        game.tick();
    }

    let [ball_x, ball_y] = game.ball().location();
    assert!(
        (ball_x - 0.5).abs() < 0.001 && (ball_y - 0.34).abs() < 0.001,
        "Ball was at location ({}, {}), but was expected at location ({}, {})",
        ball_x,
        ball_y,
        0.5,
        0.34
    );
}
