use crate::util::math::vector::Vector3;

/// An S2Point represents a point on the unit sphere as a 3D vector.  Usually
/// points are normalized to be unit length, but some methods do not require
/// this.  See util/math/vector.h for the methods available.  Among other
/// things, there are overloaded operators that make it convenient to write
/// arithmetic expressions (e.g. (1-x)*p1 + x*p2).
pub type S2Point = Vector3<f64>;
