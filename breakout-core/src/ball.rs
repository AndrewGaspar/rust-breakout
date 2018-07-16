use prelude::*;
use shape::Box;

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

    pub fn spin(&self) -> f32 {
        self.spin
    }

    pub fn set_spin(&mut self, spin: f32) {
        self.spin = spin
    }
}

impl Shape for Ball {
    fn bounding_box(&self) -> Box {
        let [mid_x, mid_y] = self.origin();
        let r = self.radius();

        Box {
            left: mid_x - r,
            right: mid_x + r,
            bottom: mid_y - r,
            top: mid_y + r,
        }
    }
}

impl Circle for Ball {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn origin(&self) -> vec2 {
        self.midpoint
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
}
