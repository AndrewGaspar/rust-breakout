use math::vec2;
use object::GameObject;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    dimensions: vec2,
    location: vec2,
}

impl Block {
    pub fn new(dimensions: vec2, location: vec2) -> Self {
        Self {
            dimensions,
            location,
        }
    }

    pub fn dimensions(&self) -> vec2 {
        self.dimensions
    }

    pub fn boundaries(&self) -> (vec2, vec2) {
        (
            (
                self.location.0 - self.dimensions.0 * 0.5,
                self.location.1 + self.dimensions.1 * 0.5,
            ),
            (
                self.location.0 + self.dimensions.0 * 0.5,
                self.location.1 - self.dimensions.1 * 0.5,
            ),
        )
    }
}

impl GameObject for Block {
    fn location(&self) -> vec2 {
        self.location
    }

    fn velocity(&self) -> vec2 {
        (0., 0.)
    }

    fn set_location(&mut self, location: vec2) {
        self.location = location;
    }

    fn set_velocity(&mut self, _: vec2) {
        panic!("A block cannot have velocity.");
    }

    fn bounding_box(&self) -> (vec2, vec2) {
        self.boundaries()
    }
}
