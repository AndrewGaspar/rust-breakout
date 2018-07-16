use super::prelude::*;
use shape::Box;

#[derive(Debug, Copy, Clone)]
pub struct Paddle {
    dimensions: vec2,
    origin: vec2,
    velocity: vec2,
}

impl Paddle {
    pub fn new(dimensions: vec2, origin: vec2) -> Self {
        Self {
            dimensions,
            origin,
            velocity: [0., 0.],
        }
    }

    pub fn dimensions(&self) -> vec2 {
        self.dimensions
    }
}

impl Shape for Paddle {
    fn bounding_box(&self) -> Box {
        Box {
            left: self.left(),
            right: self.right(),
            bottom: self.bottom(),
            top: self.top(),
        }
    }
}

impl Rectangle for Paddle {
    fn dimensions(&self) -> vec2 {
        self.dimensions
    }

    fn origin(&self) -> vec2 {
        self.origin
    }
}

impl GameObject for Paddle {
    fn location(&self) -> vec2 {
        self.origin
    }

    fn velocity(&self) -> vec2 {
        self.velocity
    }

    fn set_location(&mut self, location: vec2) {
        self.origin = location
    }

    fn set_velocity(&mut self, velocity: vec2) {
        self.velocity = velocity
    }
}
