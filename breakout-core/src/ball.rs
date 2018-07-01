use math::{vec2, Vec2};
use object::GameObject;

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    radius: f32,
    location: vec2,
    velocity: vec2,
    spin: f32,
}

impl Ball {
    pub fn new(radius: f32, location: vec2, velocity: vec2) -> Self {
        Self {
            radius,
            location,
            velocity,
            spin: 0.,
        }
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }

    pub fn spin(&self) -> f32 {
        self.spin
    }

    pub fn set_spin(&mut self, spin: f32) {
        self.spin = spin
    }
}

impl GameObject for Ball {
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
        (
            [
                self.location.x() - self.radius,
                self.location.y() + self.radius,
            ],
            [
                self.location.x() + self.radius,
                self.location.y() - self.radius,
            ],
        )
    }
}
