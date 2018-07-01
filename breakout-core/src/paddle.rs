use math::{vec2, Vec2};
use object::GameObject;

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

    pub fn boundaries(&self) -> (vec2, vec2) {
        (
            [self.origin.x(), self.origin.y() + self.dimensions.y()],
            [self.origin.x() + self.dimensions.x(), self.origin.y()],
        )
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

    fn bounding_box(&self) -> (vec2, vec2) {
        self.boundaries()
    }
}
