use object::GameObject;

pub type vec2 = (f32, f32);

pub fn next_point(start: vec2, velocity: vec2, dt: f32) -> vec2 {
    (start.0 + velocity.0 * dt, start.1 + velocity.1 * dt)
}

pub fn tick_position<G: GameObject>(obj: &mut G, dt: f32) {
    let loc = obj.location();
    let vel = obj.velocity();
    obj.set_location(next_point(loc, vel, dt));
}

pub fn overlapping_segments(a: vec2, b: vec2) -> bool {
    debug_assert!(a.1 >= a.0);
    debug_assert!(b.1 >= b.0);

    a.1 >= b.0 && b.1 >= a.0
}

#[test]
fn basic_segments() {
    assert!(overlapping_segments((1., 3.), (2., 4.)));
    assert!(overlapping_segments((1., 3.), (3., 5.)));
    assert!(!overlapping_segments((1., 3.), (4., 5.)));
    assert!(overlapping_segments((1., 5.), (2., 4.)));
}

pub fn overlapping_boxes(
    ((left_a, top_a), (right_a, bottom_a)): (vec2, vec2),
    ((left_b, top_b), (right_b, bottom_b)): (vec2, vec2),
) -> bool {
    overlapping_segments((left_a, right_a), (left_b, right_b))
        && overlapping_segments((bottom_a, top_a), (bottom_b, top_b))
}

#[test]
pub fn basic_boxes() {
    assert!(overlapping_boxes(
        ((1., 3.), (3., 1.)),
        ((2., 4.), (4., 2.))
    ));
}

pub fn objects_are_close<A: GameObject, B: GameObject>(a: &A, b: &B) -> bool {
    overlapping_boxes(a.bounding_box(), b.bounding_box())
}
