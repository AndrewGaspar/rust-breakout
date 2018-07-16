use math::{vec2, Vec2};

pub struct Box {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

pub trait Shape {
    fn bounding_box(&self) -> Box;
}

pub trait Rectangle {
    fn dimensions(&self) -> vec2;
    fn origin(&self) -> vec2;

    fn left(&self) -> f32 {
        self.origin().x()
    }

    fn right(&self) -> f32 {
        self.left() + self.dimensions().x()
    }

    fn bottom(&self) -> f32 {
        self.origin().y()
    }

    fn top(&self) -> f32 {
        self.bottom() + self.dimensions().y()
    }
}

pub trait Circle {
    fn radius(&self) -> f32;
    fn origin(&self) -> vec2;
}
