use prelude::*;
use shape::Box;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    dimensions: vec2,
    origin: vec2,
}

impl Block {
    pub fn new(dimensions: vec2, origin: vec2) -> Self {
        Self { dimensions, origin }
    }

    pub fn dimensions(&self) -> vec2 {
        self.dimensions
    }
}

impl Shape for Block {
    fn bounding_box(&self) -> Box {
        Box {
            left: self.left(),
            right: self.right(),
            bottom: self.bottom(),
            top: self.top(),
        }
    }
}

impl Rectangle for Block {
    fn dimensions(&self) -> vec2 {
        self.dimensions
    }

    fn origin(&self) -> vec2 {
        self.origin
    }
}

impl GameObject for Block {
    fn location(&self) -> vec2 {
        self.origin
    }

    fn velocity(&self) -> vec2 {
        [0., 0.]
    }

    fn set_location(&mut self, origin: vec2) {
        self.origin = origin;
    }

    fn set_velocity(&mut self, _: vec2) {
        panic!("A block cannot have velocity.");
    }
}
