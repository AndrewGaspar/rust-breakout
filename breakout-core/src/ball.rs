use math::{vec2, Vec2};
use object::GameObject;

#[derive(Debug, Copy, Clone)]
pub struct Ball {
    radius: f32,
    midpoint: vec2,
    velocity: vec2,
    spin: f32,
}

impl Ball {
    pub fn new(radius: f32, midpoint: vec2, velocity: vec2) -> Self {
        Self {
            radius,
            midpoint,
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
        self.midpoint
    }

    fn velocity(&self) -> vec2 {
        self.velocity
    }

    fn set_location(&mut self, location: vec2) {
        self.midpoint = location
    }

    fn set_velocity(&mut self, velocity: vec2) {
        self.velocity = velocity
    }

    fn bounding_box(&self) -> (vec2, vec2) {
        (
            [
                self.midpoint.x() - self.radius,
                self.midpoint.y() + self.radius,
            ],
            [
                self.midpoint.x() + self.radius,
                self.midpoint.y() - self.radius,
            ],
        )
    }
}
