use std::cmp::Ordering;

use super::s2cell_id::S2CellId;

///An S2Cell is an S2Region object that represents a cell.  Unlike S2CellIds,
/// it supports efficient containment and intersection tests.  However, it is
/// also a more expensive representation (currently 48 bytes rather than 8).
#[derive(Debug, Clone)]
pub struct S2Cell {
    face: i8,
    level: i8,
    orientation: i8,
    id: S2CellId,
}

enum BoundaryEdge {
    Bottom = 0,
    Right = 1,
    Top = 2,
    Left = 3,
}

impl S2Cell {
    pub fn new(id: S2CellId) -> Self {
        unimplemented!()
    }

    pub fn from_face(face: i8) -> Self {
        unimplemented!()
    }

    pub fn id(&self) -> S2CellId {
        self.id
    }

    pub fn face(&self) -> i8 {
        self.face
    }

    pub fn level(&self) -> i8 {
        self.level
    }

    pub fn orientation(&self) -> i8 {
        self.orientation
    }

    pub fn is_leaf(&self) -> bool {
        self.level == S2CellId::MAX_LEVEL
    }
}

impl PartialEq for S2Cell {
    fn eq(&self, other: &S2Cell) -> bool {
        self.id == other.id
    }
}

impl Eq for S2Cell {}

impl PartialOrd for S2Cell {
    fn partial_cmp(&self, other: &S2Cell) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl Ord for S2Cell {
    fn cmp(&self, other: &S2Cell) -> Ordering {
        self.id.cmp(&other.id)
    }
}
