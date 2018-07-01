use math::{vec2, Vec2};
use object::GameObject;

#[derive(Debug, Copy, Clone)]
pub struct Paddle {
    dimensions: vec2,
    location: vec2,
    velocity: vec2,
}

impl Paddle {
    pub fn new(dimensions: vec2, location: vec2) -> Self {
        Self {
            dimensions,
            location,
            velocity: [0., 0.],
        }
    }

    pub fn dimensions(&self) -> vec2 {
        self.dimensions
    }

    pub fn boundaries(&self) -> (vec2, vec2) {
        (
            [
                self.location.x() - self.dimensions.x() * 0.5,
                self.location.y() + self.dimensions.y() * 0.5,
            ],
            [
                self.location.x() + self.dimensions.x() * 0.5,
                self.location.y() - self.dimensions.y() * 0.5,
            ],
        )
    }
}

impl GameObject for Paddle {
    fn location(&self) -> vec2 {
        self.location
    }

    fn velocity(&self) -> vec2 {
        self.velocity
    }

    fn set_location(&mut self, location: vec2) {
        self.location = location
    }

    fn set_velocity(&mut self, velocity: vec2) {
        self.velocity = velocity
    }

    fn bounding_box(&self) -> (vec2, vec2) {
        self.boundaries()
    }
}
