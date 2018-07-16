use math::vec2;

pub trait GameObject {
    fn location(&self) -> vec2;
    fn velocity(&self) -> vec2;

    fn set_location(&mut self, location: vec2);
    fn set_velocity(&mut self, velocity: vec2);
}
