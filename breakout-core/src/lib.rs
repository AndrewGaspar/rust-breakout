extern crate rand;

mod ball;
mod block;
mod game;
mod math;
mod object;
mod paddle;

#[cfg(test)]
mod tests;

pub use ball::Ball;
pub use block::Block;
pub use game::{Breakout, BreakoutBuilder};
pub use object::GameObject;
pub use paddle::Paddle;
