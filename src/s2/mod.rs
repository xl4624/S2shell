use num_traits::ToPrimitive;

use crate::r2::R2Point;

// This file contains documentation of the various coordinate systems used
// throughout the library. Most importantly, S2 defines a framework for
// decomposing the unit sphere into a hierarchy of "cells". Each cell is a
// quadrilateral bounded by four geodesics. The top level of the hierarchy is
// obtained by projecting the six faces of a cube onto the unit sphere, and
// lower levels are obtained by subdividing each cell into four children
// recursively. Cells are numbered such that sequentially increasing cells
// follow a continuous space-filling curve over the entire sphere. The
// transformation is designed to make the cells at each level fairly uniform
// in size.
//
////////////////////////// S2Cell Decomposition /////////////////////////
//
// The following methods define the cube-to-sphere projection used by
// the S2Cell decomposition.
//
// In the process of converting a latitude-longitude pair to a 64-bit cell
// id, the following coordinate systems are used:
//
//  (id)
//    An S2CellId is a 64-bit encoding of a face and a Hilbert curve position
//    on that face. The Hilbert curve position implicitly encodes both the
//    position of a cell and its subdivision level (see s2cell_id.h).
//
//  (face, i, j)
//    Leaf-cell coordinates. "i" and "j" are integers in the range
//    [0,(2**30)-1] that identify a particular leaf cell on the given face.
//    The (i, j) coordinate system is right-handed on each face, and the
//    faces are oriented such that Hilbert curves connect continuously from
//    one face to the next.
//
//  (face, s, t)
//    Cell-space coordinates. "s" and "t" are real numbers in the range
//    [0,1] that identify a point on the given face. For example, the point
//    (s, t) = (0.5, 0.5) corresponds to the center of the top-level face
//    cell. This point is also a vertex of exactly four cells at each
//    subdivision level greater than zero.
//
//  (face, si, ti)
//    Discrete cell-space coordinates. These are obtained by multiplying
//    "s" and "t" by 2**31 and rounding to the nearest unsigned integer.
//    Discrete coordinates lie in the range [0,2**31]. This coordinate
//    system can represent the edge and center positions of all cells with
//    no loss of precision (including non-leaf cells). In binary, each
//    coordinate of a level-k cell center ends with a 1 followed by
//    (30 - k) 0s. The coordinates of its edges end with (at least)
//    (31 - k) 0s.
//
//  (face, u, v)
//    Cube-space coordinates in the range [-1,1].  To make the cells at each
//    level more uniform in size after they are projected onto the sphere,
//    we apply a nonlinear transformation of the form u=f(s), v=f(t).
//    The (u, v) coordinates after this transformation give the actual
//    coordinates on the cube face (modulo some 90 degree rotations) before
//    it is projected onto the unit sphere.
//
//  (face, u, v, w)
//    Per-face coordinate frame. This is an extension of the (face, u, v)
//    cube-space coordinates that adds a third axis "w" in the direction of
//    the face normal. It is always a right-handed 3D coordinate system.
//    Cube-space coordinates can be converted to this frame by setting w=1,
//    while (u,v,w) coordinates can be projected onto the cube face by
//    dividing by w, i.e.(face, u/w, v/w).
//
//  (x, y, z)
//    Direction vector (S2Point). Direction vectors are not necessarily unit
//    length, and are often chosen to be points on the biunit cube
//    [-1,+1]x[-1,+1]x[-1,+1]. They can be be normalized to obtain the
//    corresponding point on the unit sphere.
//
//  (lat, lng)
//    Latitude and longitude (S2LatLng). Latitudes must be between -90 and
//    90 degrees inclusive, and longitudes must be between -180 and 180
//    degrees inclusive.
//
// Note that the (i, j), (s, t), (si, ti), and (u, v) coordinate systems are
// right-handed on all six faces.

/// The maximum absolute error in U/V coordinates when converting from XYZ.
///
/// The XYZ -> UV conversion is a single division per coordinate, which is
/// promised to be at most 0.5*DBL_EPSILON absolute error for values with
/// magnitude less than two.
pub const MAX_XYZ_TO_UV_ERROR: f64 = 0.5 * f64::EPSILON;

/// This is the number of levels needed to specify a leaf cell. This
/// constant is defined here so that the S2::Metric class and the conversion
/// functions below can be implemented without including s2cell_id.h. Please
/// see s2cell_id.h for other useful constants and conversion functions.
pub const MAX_CELL_LEVEL: i32 = 30;

/// The maximum index of a valid leaf cell plus one. The range of valid leaf
/// cell indices is [0..LIMIT_IJ-1].
pub const LIMIT_IJ: i32 = 1 << MAX_CELL_LEVEL; // == S2CellId::MAX_SIZE

/// The maximum value of an si- or ti-coordinate. The range of valid (si,ti)
/// values is [0..MAX_SITI].
pub const MAX_SITI: u32 = 1 << (MAX_CELL_LEVEL + 1);

/// Convert an s- or t-value to the corresponding u- or v-value. This is
/// a non-linear transformation from [0,1] to [-1,1] that attempts to
/// make the cell sizes more uniform.
pub fn st_to_uv(s: f64) -> f64 {
    if s >= 0.5 {
        (1.0 / 3.0) * (4.0 * s * s - 1.0)
    } else {
        (1.0 / 3.0) * (1.0 - 4.0 * (1.0 - s) * (1.0 - s))
    }
}

/// The inverse of the STtoUV transformation. Note that it is not always
/// true that UVtoST(STtoUV(x)) == x due to numerical errors.
pub fn uv_to_st(u: f64) -> f64 {
    if u >= 0.0 {
        0.5 * (1.0 + 3.0 * u).sqrt()
    } else {
        1.0 - 0.5 * (1.0 - 3.0 * u).sqrt()
    }
}

/// Convert the i- or j-index of a leaf cell to the minimum corresponding s-
/// or t-value contained by that cell. The argument must be in the range
/// [0..2**30], i.e. up to one position beyond the normal range of valid leaf
/// cell indices.
pub fn ij_to_st_min(i: i32) -> f64 {
    debug_assert!((0..=LIMIT_IJ).contains(&i));

    (1.0 / LIMIT_IJ as f64) * (i as f64)
}

/// Return the i- or j-index of the leaf cell containing the given
/// s- or t-value. If the argument is outside the range spanned by valid
/// leaf cell indices, return the index of the closest valid leaf cell (i.e.,
/// return values are clamped to the range of valid leaf cell indices).
///
/// # Examples
/// ```
/// use s2shell::s2::{st_to_ij, LIMIT_IJ};
///
/// let s = 0.5_f64;
/// assert_eq!(st_to_ij(s), LIMIT_IJ / 2);
/// ```
pub fn st_to_ij(s: f64) -> i32 {
    let index = (LIMIT_IJ as f64 * s - 0.5).round() as i32;
    index.clamp(0, LIMIT_IJ - 1)
}

/// Convert an si- or ti-value to the corresponding s- or t-value.
pub fn siti_to_st(si: u32) -> f64 {
    debug_assert!(si < MAX_SITI);

    (1.0 / MAX_SITI as f64) * (si as f64)
}

/// Return the si- or ti-coordinate that is nearest to the given s- or
/// t-value. The result may be outside the range of valid (si,ti)-values.
pub fn st_to_siti(s: f64) -> u32 {
    (s * MAX_SITI as f64)
        .round()
        .to_u32()
        .expect("st_to_siti: error when converting")
}

/// Convert (face, u, v) coordinates to a direction vector (not
/// necessarily unit length).
#[rustfmt::skip]
pub fn face_uv_to_xyz(face: i32, u: f64, v: f64) -> S2Point {
    match face {
        0 => S2Point::new( 1.0,    u,    v),
        1 => S2Point::new(  -u,  1.0,    v),
        2 => S2Point::new(  -u,   -v,  1.0),
        3 => S2Point::new(-1.0,   -v,   -u),
        4 => S2Point::new(   v, -1.0,   -u),
        5 => S2Point::new(   v,    u, -1.0),
        _ => panic!("invalid face: {face}"),
    }
}

pub fn face_uv_to_xyz_from_r2point(face: i8, uv: &R2Point) -> S2Point {
    todo!()
}

/// Return the face containing the given direction vector. (For points on
/// the boundary between faces, the result is arbitrary but repeatable.)
pub fn get_face(p: &S2Point) -> i32 {
    let face = p.largest_abs_component();
    if p[face as usize] < 0.0 {
        face + 3
    } else {
        face
    }
}

pub mod s2cap;
pub mod s2cell;
pub mod s2cell_id;
pub mod s2latlng_rect;
pub mod s2point;
pub mod s2region;

pub use s2cap::S2Cap;
pub use s2cell::S2Cell;
pub use s2cell_id::S2CellId;
pub use s2latlng_rect::S2LatLngRect;
pub use s2point::S2Point;
pub use s2region::S2Region;

mod internal;
