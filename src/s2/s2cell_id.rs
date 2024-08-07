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
pub struct S2CellId(u64);

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

    pub fn new(id: u64) -> Self {
        S2CellId(id)
    }

    /// Returns an invalid cell id.
    pub fn none() -> Self {
        S2CellId(0)
    }

    /// Returns an invalid cell id guaranteed to be larger than any valid cell id. Useful for creating indexes
    pub fn sentinel() -> Self {
        S2CellId(u64::MAX)
    }
}
