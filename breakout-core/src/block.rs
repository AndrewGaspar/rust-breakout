use math::{vec2, Vec2};
use object::GameObject;

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

    pub fn boundaries(&self) -> (vec2, vec2) {
        (
            [self.origin.x(), self.origin.y() + self.dimensions.y()],
            [self.origin.x() + self.dimensions.x(), self.origin.y()],
        )
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

    fn bounding_box(&self) -> (vec2, vec2) {
        self.boundaries()
    }
}
