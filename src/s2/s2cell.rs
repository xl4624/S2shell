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

use crate::s2::s2cell_id::S2CellId;

/// An S2Cell is an S2Region object that represents a cell. Unlike S2CellId's,
/// it supports efficient containment and intersection tests. However, it is
/// also a more expensive representation.
#[derive(Debug, Clone)]
pub struct S2Cell {
    id: S2CellId,
    face: i32,
    level: i32,
    orientation: i32,
}

enum BoundaryEdge {
    Bottom = 0,
    Right = 1,
    Top = 2,
    Left = 3,
}

impl S2Cell {
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
    pub fn new(id: S2CellId) -> Self {
        let (face, i, j, orientation) = id.to_face_ij_orientation();
        S2Cell {
            id,
            face,
            orientation,
            level: id.level(),
        }
    }

    pub fn from_face(face: i32) -> Self {
        todo!()
    }

    pub fn id(&self) -> S2CellId {
        self.id
    }

    pub fn face(&self) -> i32 {
        self.face
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn orientation(&self) -> i32 {
        self.orientation
    }

    pub fn is_leaf(&self) -> bool {
        self.level == S2CellId::MAX_LEVEL
    }
}
