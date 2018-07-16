use prelude::*;
use shape;

#[allow(non_camel_case_types)]
pub type vec2 = [f32; 2];

pub trait Vec2 {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl Vec2 for vec2 {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }
}

pub fn next_point(start: vec2, velocity: vec2, dt: f32) -> vec2 {
    [start.x() + velocity.x() * dt, start.y() + velocity.y() * dt]
}

pub fn tick_position<G: GameObject>(obj: &mut G, dt: f32) {
    let loc = obj.location();
    let vel = obj.velocity();
    obj.set_location(next_point(loc, vel, dt));
}

pub fn overlapping_segments(a: vec2, b: vec2) -> bool {
    debug_assert!(a[1] >= a[0]);
    debug_assert!(b[1] >= b[0]);

    a[1] >= b[0] && b[1] >= a[0]
}

#[test]
fn basic_segments() {
    assert!(overlapping_segments((1., 3.), (2., 4.)));
    assert!(overlapping_segments((1., 3.), (3., 5.)));
    assert!(!overlapping_segments((1., 3.), (4., 5.)));
    assert!(overlapping_segments((1., 5.), (2., 4.)));
}

pub fn overlapping_boxes(a: &shape::Box, b: &shape::Box) -> bool {
    overlapping_segments([a.left, a.right], [b.left, b.right])
        && overlapping_segments([a.bottom, a.top], [b.bottom, b.top])
}

#[test]
pub fn basic_boxes() {
    assert!(overlapping_boxes(
        ((1., 3.), (3., 1.)),
        ((2., 4.), (4., 2.))
    ));
}

pub fn objects_are_close<A: Shape, B: Shape>(a: &A, b: &B) -> bool {
    overlapping_boxes(&a.bounding_box(), &b.bounding_box())
}
