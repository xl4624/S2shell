use crate::s2::s2point::S2Point;

/// An S2CellId is a 64-bit unsigned integer that uniquely identifies a
/// cell in the S2 cell decomposition.  It has the following format:
///
///   id = [face][face_pos]
///
///   face:     a 3-bit number (range 0..5) encoding the cube face.
///
///   face_pos: a 61-bit number encoding the position of the center of this
///             cell along the Hilbert curve over this face (see the Wiki
///             pages for details).
///
/// Sequentially increasing cell ids follow a continuous space-filling curve
/// over the entire sphere.  They have the following properties:
///
///  - The id of a cell at level k consists of a 3-bit face number followed
///    by k bit pairs that recursively select one of the four children of
///    each cell.  The next bit is always 1, and all other bits are 0.
///    Therefore, the level of a cell is determined by the position of its
///    lowest-numbered bit that is turned on (for a cell at level k, this
///    position is 2 * (kMaxLevel - k).)
///
///  - The id of a parent cell is at the midpoint of the range of ids spanned
///    by its children (or by its descendants at any level).
///
/// Leaf cells are often used to represent points on the unit sphere, and
/// this class provides methods for converting directly between these two
/// representations.  For cells that represent 2D regions rather than
/// discrete point, it is better to use the S2Cell class.
///
/// All methods require `is_valid()` to be true unless otherwise specified
/// (although not all methods enforce this).
///
/// This class is intended to be copied by value as desired.  It uses
/// the default copy constructor and assignment operator.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct S2CellId {
    id: u64,
}

impl S2CellId {
    // Although only 60 bits are needed to represent the index of a leaf cell, the
    // extra position bit lets us encode each cell as its Hilbert curve position
    // at the cell center, which is halfway along the portion of the Hilbert curve
    // that fills that cell.
    pub const FACE_BITS: i8 = 3;
    pub const NUM_FACES: i8 = 6;
    pub const MAX_LEVEL: i8 = 30;
    pub const POS_BITS: i8 = 2 * S2CellId::MAX_LEVEL + 1;
    pub const MAX_SIZE: i32 = 1 << S2CellId::MAX_LEVEL;

    pub fn new(id: u64) -> S2CellId {
        S2CellId { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn face(&self) -> i8 {
        (self.id >> Self::POS_BITS) as i8
    }

    pub fn pos(&self) -> u64 {
        todo!()
    }

    pub fn level(&self) -> i8 {
        todo!()
    }

    /// Returns an invalid cell id.
    pub fn none() -> S2CellId {
        S2CellId::new(0)
    }

    /// Returns an invalid cell id guaranteed to be larger than any valid cell id. Useful for creating indexes
    pub fn sentinel() -> S2CellId {
        S2CellId::new(u64::MAX)
    }

    pub fn from_face(face: i8) -> S2CellId {
        todo!()
    }

    pub fn from_face_pos_level(face: i8, pos: u64, level: i8) -> S2CellId {
        todo!()
    }


    fn to_point_raw(&self) -> S2Point {
        todo!();
    }

    // pub fn get_center_st() -> R2Point {
    //     todo!()
    // }

    pub fn get_size_st() -> f64 {
        todo!()
    }

    pub fn get_size_st_at_level(level: i8) -> f64 {
        todo!()
    }

    // pub fn get_bound_st() -> R2Rect {
    //     todo!()
    // }

    // pub fn get_center_uv(&self) -> R2Point {
    //     todo!()
    // }

    // pub fn get_bound_uv(&self) -> R2Rect {
    //     todo!()
    // }

    // pub fn expanded_by_distance_uv(&self, uv: &R2Rect, distance: S1Angle) -> R2Rect {
    //     todo!()
    // }

    pub fn get_center_siti(psi: i32, pti: i32) -> i32 {
        todo!()
    }

    pub fn is_valid(&self) -> bool {
        const LSB_MASK: u64 = 0x1555555555555555;
        self.face() < Self::NUM_FACES && (self.lsb() & LSB_MASK) != 0
    }

    pub fn lsb(&self) -> u64 {
        self.id & (!self.id + 1)
    }

    pub fn lsb_for_level(&self, level: i8) -> u64 {
            1_u64 << (2 * (Self::MAX_LEVEL - level))
    }
}

impl Into<S2Point> for S2CellId {
    fn into(self) -> S2Point {
        self.to_point_raw().normalize()
    }
}

// impl Into<S2LatLng> for S2CellId {
//     fn into(self) -> S2LatLng {
//         todo!()
//     }
// }

