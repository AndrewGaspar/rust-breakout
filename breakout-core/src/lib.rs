extern crate rand;

mod ball;
mod block;
mod game;
mod math;
mod object;
mod paddle;
mod shape;

#[cfg(test)]
mod tests;

pub use ball::Ball;
pub use block::Block;
pub use game::{Breakout, BreakoutBuilder};
pub use math::{vec2, Vec2};
pub use object::GameObject;
pub use paddle::Paddle;
pub use shape::{Circle, Rectangle, Shape};

pub mod prelude {
    pub use super::{
        vec2, Ball, Block, Breakout, BreakoutBuilder, Circle, GameObject, Paddle, Rectangle, Shape,
        Vec2,
    };
}
