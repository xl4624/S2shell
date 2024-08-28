// Copyright 2005 Google Inc. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS-IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// Original Author: ericv@google.com (Eric Veach)

use lazy_static::lazy_static;
use num_traits::ToPrimitive;

use crate::{
    r2::R2Rect,
    s2::{
        face_siti_to_xyz, ij_to_st_min,
        internal::{INVERT_MASK, POS_TO_IJ, POS_TO_ORIENTATION, SWAP_MASK},
        s2point::S2Point,
        st_to_uv,
    },
};

/// An S2CellId is a 64-bit unsigned integer that uniquely identifies a
/// cell in the S2 cell decomposition. It has the following format:
///
///   id = [face][face_pos]
///
///   face:     a 3-bit number (range 0..5) encoding the cube face.
///
///   face_pos: a 61-bit number encoding the position of the center of this
///             cell along the Hilbert curve over this face.
///
/// Sequentially increasing cell ids follow a continuous space-filling curve
/// over the entire sphere. They have the following properties:
///
///  - The id of a cell at level k consists of a 3-bit face number followed
///    by k bit pairs that recursively select one of the four children of
///    each cell. The next bit is always 1, and all other bits are 0.
///    Therefore, the level of a cell is determined by the position of its
///    lowest-numbered bit that is turned on (for a cell at level k, this
///    position is 2 * (S2CellId::MAX_LEVEL - k).)
///
///  - The id of a parent cell is at the midpoint of the range of ids spanned
///    by its children (or by its descendants at any level).
///
/// Leaf cells are often used to represent points on the unit sphere, and
/// this class provides methods for converting directly between these two
/// representations. For cells that represent 2D regions rather than
/// discrete point, it is better to use the S2Cell class.
///
/// All methods require `is_valid()` to be true unless otherwise specified
/// (although not all methods enforce this).
///
/// This class is intended to be copied by value as desired.
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct S2CellId {
    id: u64,
}

/// The following lookup tables are used to convert efficiently between an
/// (i,j) cell index and the corresponding position along the Hilbert curve.
/// "lookup_pos" maps 4 bits of "i", 4 bits of "j", and 2 bits representing the
/// orientation of the current cell into 8 bits representing the order in which
/// that subcell is visited by the Hilbert curve, plus 2 bits indicating the
/// new orientation of the Hilbert curve within that subcell. (Cell
/// orientations are represented as combination of s2::internal::SWAP_MASK and
/// s2::internal::INVERT_MASK.)
///
/// "lookup_ij" is an inverted table used for mapping in the opposite
/// direction.
const LOOKUP_BITS: i32 = 4;
const LOOKUP_TABLE_SIZE: usize = 1 << (2 * LOOKUP_BITS + 2);

lazy_static! {
    static ref LOOKUP_TABLES: ([u16; LOOKUP_TABLE_SIZE], [u16; LOOKUP_TABLE_SIZE]) =
        init_lookup_tables();
    static ref LOOKUP_POS: &'static [u16; LOOKUP_TABLE_SIZE] = &LOOKUP_TABLES.0;
    static ref LOOKUP_IJ: &'static [u16; LOOKUP_TABLE_SIZE] = &LOOKUP_TABLES.1;
}

#[rustfmt::skip]
fn init_lookup_tables() -> ([u16; LOOKUP_TABLE_SIZE], [u16; LOOKUP_TABLE_SIZE]) {
    let mut lookup_pos = [0u16; LOOKUP_TABLE_SIZE];
    let mut lookup_ij = [0u16; LOOKUP_TABLE_SIZE];

    // I'm leaving this with too many arguments because I'm trying to
    // replicate the original library's implementation as closely as I
    // can because I don't understand this enough to make this iterative.
    #[allow(clippy::too_many_arguments)]
    fn init_lookup_cell(
        level: i32, i: i32, j: i32, orig_orientation: i32, pos: i32, orientation: i32,
        lookup_pos: &mut [u16; LOOKUP_TABLE_SIZE], lookup_ij: &mut [u16; LOOKUP_TABLE_SIZE]
    ) {
        if level == LOOKUP_BITS {
            let ij: i32 = (i << LOOKUP_BITS) + j;
            lookup_pos[((ij << 2) + orig_orientation) as usize] = ((pos << 2) + orientation) as u16;
            lookup_ij[((pos << 2) + orig_orientation) as usize] = ((ij << 2) + orientation) as u16;
        } else {
            let level = level + 1;
            let i = i << 1;
            let j = j << 1;
            let pos = pos << 2;
            let r: [i32; 4] = POS_TO_IJ[orientation as usize];
            init_lookup_cell(level, i + (r[0] >> 1), j + (r[0] & 1), orig_orientation, pos,
                orientation ^ POS_TO_ORIENTATION[0], lookup_pos, lookup_ij);
            init_lookup_cell(level, i + (r[1] >> 1), j + (r[1] & 1), orig_orientation, pos + 1,
                orientation ^ POS_TO_ORIENTATION[1], lookup_pos, lookup_ij);
            init_lookup_cell(level, i + (r[2] >> 1), j + (r[2] & 1), orig_orientation, pos + 2,
                orientation ^ POS_TO_ORIENTATION[2], lookup_pos, lookup_ij);
            init_lookup_cell(level, i + (r[3] >> 1), j + (r[3] & 1), orig_orientation, pos + 3,
                orientation ^ POS_TO_ORIENTATION[3], lookup_pos, lookup_ij);
        }
    }

    init_lookup_cell(0, 0, 0, 0, 0, 0, &mut lookup_pos, &mut lookup_ij);
    init_lookup_cell(0, 0, 0, SWAP_MASK, 0, SWAP_MASK, &mut lookup_pos, &mut lookup_ij);
    init_lookup_cell(0, 0, 0, INVERT_MASK, 0, INVERT_MASK, &mut lookup_pos, &mut lookup_ij);
    init_lookup_cell(0, 0, 0, SWAP_MASK | INVERT_MASK, 0, SWAP_MASK | INVERT_MASK, &mut lookup_pos, &mut lookup_ij);

    (lookup_pos, lookup_ij)
}

impl S2CellId {
    // Although only 60 bits are needed to represent the index of a leaf cell, the
    // extra position bit lets us encode each cell as its Hilbert curve position
    // at the cell center, which is halfway along the portion of the Hilbert curve
    // that fills that cell.
    pub const FACE_BITS: i32 = 3;
    pub const NUM_FACES: i32 = 6;
    pub const MAX_LEVEL: i32 = 30;
    pub const POS_BITS: i32 = 2 * S2CellId::MAX_LEVEL + 1;
    pub const MAX_SIZE: i32 = 1 << S2CellId::MAX_LEVEL;

    /// Creates a new S2CellId from a 64-bit unsigned integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::s2::s2cell_id::S2CellId;
    ///
    /// let cell_id = S2CellId::new(123456789);
    /// assert_eq!(cell_id.id(), 123456789);
    /// ```
    pub fn new(id: u64) -> S2CellId {
        S2CellId { id }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    /// Which cube face this cell belongs to, in the range 0..5.
    pub fn face(&self) -> i32 {
        (self.id >> S2CellId::POS_BITS) as i32
    }

    /// The position of the cell center along the Hilbert curve over this face,
    /// in the range 0..(2**kPosBits-1).
    pub fn pos(&self) -> u64 {
        self.id & (0_u64 >> S2CellId::FACE_BITS)
    }

    pub fn level(&self) -> i32 {
        debug_assert!(self.id != 0_u64);
        todo!()
        // S2CellId::MAX_LEVEL - (Bt
    }

    /// Returns an invalid cell id.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::s2::s2cell_id::S2CellId;
    ///
    /// let invalid = S2CellId::none();
    /// assert!(!invalid.is_valid());
    /// ```
    pub fn none() -> S2CellId {
        S2CellId::new(0)
    }

    /// Returns an invalid cell id guaranteed to be larger than any valid cell id. Useful for creating indexes
    pub fn sentinel() -> S2CellId {
        S2CellId::new(u64::MAX)
    }

    pub fn from_face(face: i32) -> S2CellId {
        todo!()
    }

    pub fn from_face_pos_level(face: i8, pos: u64, level: i8) -> S2CellId {
        todo!()
    }

    fn to_point_raw(self) -> S2Point {
        let (face, si, ti) = self.get_center_siti();
        face_siti_to_xyz(face, si.to_u32().unwrap(), ti.to_u32().unwrap())
    }

    // pub fn get_center_st() -> R2Point {
    //     todo!()
    // }

    pub fn get_size_st(&self) -> f64 {
        S2CellId::get_size_st_at_level(self.level())
    }

    pub fn get_size_st_at_level(level: i32) -> f64 {
        ij_to_st_min(S2CellId::get_size_ij_at_level(level))
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

    pub fn get_center_siti(&self) -> (i32, i32, i32) {
        // First we compute the discrete (i,j) coordinates of a leaf cell contained
        // within the given cell.  Given that cells are represented by the Hilbert
        // curve position corresponding at their center, it turns out that the cell
        // returned by ToFaceIJOrientation is always one of two leaf cells closest
        // to the center of the cell (unless the given cell is a leaf cell itself,
        // in which case there is only one possibility).
        //
        // Given a cell of size s >= 2 (i.e. not a leaf cell), and letting (imin,
        // jmin) be the coordinates of its lower left-hand corner, the leaf cell
        // returned by ToFaceIJOrientation() is either (imin + s/2, jmin + s/2)
        // (imin + s/2 - 1, jmin + s/2 - 1).  The first case is the one we want.
        // We can distinguish these two cases by looking at the low bit of "i" or
        // "j".  In the second case the low bit is one, unless s == 2 (i.e. the
        // level just above leaf cells) in which case the low bit is zero.
        //
        // In the code below, the expression ((i ^ (int(id_) >> 2)) & 1) is true
        // if we are in the second case described above.
        let (face, i, j, _) = self.to_face_ij_orientation();
        let delta: i32 = if self.is_leaf() {
            1
        } else if (i ^ (self.id.to_i32().unwrap() >> 2)) & 1 != 0 {
            2
        } else {
            0
        };
        let psi = 2 * i + delta;
        let pti = 2 * j + delta;

        (face, psi, pti)
    }

    /// Return the edge length of this cell in (i,j)-space.
    pub fn get_size_ij(&self) -> i32 {
        S2CellId::get_size_ij_at_level(self.level())
    }

    /// Like `get_size_ij`, but return the size of cells at the given level.
    pub fn get_size_ij_at_level(level: i32) -> i32 {
        debug_assert!(level > 0);
        debug_assert!(level < S2CellId::MAX_LEVEL);
        1 << (S2CellId::MAX_LEVEL - level)
    }

    /// Return true if id() represents a valid cell.
    ///
    /// All methods require is_valid() to be true unless otherwise specified
    /// (although not all methods enforce this).
    pub fn is_valid(&self) -> bool {
        self.face() < S2CellId::NUM_FACES && (self.lsb() & 0x1555555555555555) != 0
    }

    /// Return true if this is a leaf cell (more efficient than checking
    /// whether level() == kMaxLevel).
    pub fn is_leaf(&self) -> bool {
        (self.id & 1) != 0
    }

    /// Return true if this is a top-level face cell (more efficient than
    /// checking whether level() == 0).
    pub fn is_face(&self) -> bool {
        self.id &
    }

    /// Converts this cell ID to face, i, j, and orientation.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::s2::s2cell_id::S2CellId;
    ///
    /// let cell_id = S2CellId::new(0x1234567890ABCDEF);
    /// let (face, i, j, orientation) = cell_id.to_face_ij_orientation();
    /// assert!(face >= 0 && face < 6);
    /// assert!(orientation >= 0 && orientation < 4);
    /// ```
    pub fn to_face_ij_orientation(&self) -> (i32, i32, i32, i32) {
        let (mut i, mut j) = (0, 0);
        let face = self.face();
        let mut bits = face & SWAP_MASK;

        fn get_bits(k: i32, id: u64, bits: &mut i32, i: &mut i32, j: &mut i32, lookup_ij: &[u16]) {
            let nbits = if k == 7 {
                S2CellId::MAX_LEVEL - 7 * LOOKUP_BITS
            } else {
                LOOKUP_BITS
            };
            *bits += (((id >> (k * 2 * LOOKUP_BITS + 1)) & ((1 << (2 * nbits)) - 1)) as i32) << 2;
            *bits = lookup_ij[*bits as usize] as i32;
            *i += (*bits >> (LOOKUP_BITS + 2)) << (k * LOOKUP_BITS);
            *j += ((*bits >> 2) & ((1 << LOOKUP_BITS) - 1)) << (k * LOOKUP_BITS);
            *bits &= SWAP_MASK | INVERT_MASK;
        }

        for k in (0..8).rev() {
            get_bits(k, self.id, &mut bits, &mut i, &mut j, &LOOKUP_IJ[..]);
        }

        debug_assert_eq!(0, POS_TO_ORIENTATION[2]);
        debug_assert_eq!(SWAP_MASK, POS_TO_ORIENTATION[0]);
        let orientation = if self.lsb() & 0x1111111111111110 != 0 {
            bits ^ SWAP_MASK
        } else {
            bits
        };

        (face, i, j, orientation)
    }

    /// Return the lowest-numbered bit that is on for this cell id, which is
    /// equal to (uint64_t{1} << (2 * (kMaxLevel - level))).  So for example,
    /// a.lsb() <= b.lsb() if and only if a.level() >= b.level(), but the
    /// first test is more efficient.
    pub fn lsb(&self) -> u64 {
        self.id & (self.id.wrapping_neg() + 1)
    }

    pub fn lsb_for_level(&self, level: i32) -> u64 {
        1_u64 << (2 * (S2CellId::MAX_LEVEL - level))
    }

    /// Return the bound in (u,v)-space for the cell at the given level containing
    /// the leaf cell with the given (i,j)-coordinates.
    pub fn ij_level_to_bound_uv(i: i32, j: i32, level: i32) -> R2Rect {
        let cell_size = S2CellId::get_size_ij_at_level(level);
        let mut bound = R2Rect::default();
        for d in 0..2 {
            let ij = if d == 0 { i } else { j };
            let ij_lo = ij & -cell_size;
            let ij_hi = ij_lo + cell_size;
            bound[d][0] = st_to_uv(ij_to_st_min(ij_lo));
            bound[d][1] = st_to_uv(ij_to_st_min(ij_hi));
        }
        bound
    }
}

impl From<S2CellId> for S2Point {
    fn from(val: S2CellId) -> S2Point {
        val.to_point_raw().normalize()
    }
}

// impl Into<S2LatLng> for S2CellId {
//     fn into(self) -> S2LatLng {
//         todo!()
//     }
// }
