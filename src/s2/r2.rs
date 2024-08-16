use crate::util::math::Vector2;

pub type R2Point = Vector2<f64>;

/// An edge in R2 space.
#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct R2Edge {
    v0: R2Point,
    v1: R2Point,
}

impl R2Edge {
    pub fn new(v0: &R2Point, v1: &R2Point) -> R2Edge {
        R2Edge { v0: *v0, v1: *v1 }
    }
}
