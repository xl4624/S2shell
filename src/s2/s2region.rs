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

use crate::s2::{
    s2cap::S2Cap, s2cell::S2Cell, s2cell_id::S2CellId, s2latlng_rect::S2LatLngRect,
    s2point::S2Point,
};

/// An S2Region represents a two-dimensional region over the unit sphere.
/// It is an abstract interface with various concrete subtypes.
///
/// The main purpose of this interface is to allow complex regions to be
/// approximated as simpler regions.  So rather than having a wide variety
/// of virtual methods that are implemented by all subtypes, the interface
/// is restricted to methods that are useful for computing approximations.
pub trait S2Region {
    // Returns a deep copy of the region.
    //
    // Note that each subtype of S2Region returns an object of its own type.
    fn clone(&self) -> Self;

    // Returns a bounding spherical cap that contains the region.  The bound may
    // not be tight.
    fn get_cap_bound(&self) -> S2Cap;

    // Returns a bounding latitude-longitude rectangle that contains the region.
    // The bound may not be tight.
    fn get_rect_bound(&self) -> S2LatLngRect;

    // Returns a small collection of S2CellIds whose union covers the region.
    // The cells are not sorted, may have redundancies (such as cells that
    // contain other cells), and may cover much more area than necessary.
    //
    // This method is not intended for direct use by client code.  Clients
    // should typically use S2RegionCoverer::GetCovering, which has options to
    // control the size and accuracy of the covering.  Alternatively, if you
    // want a fast covering and don't care about accuracy, consider calling
    // S2RegionCoverer::GetFastCovering (which returns a cleaned-up version of
    // the covering computed by this method).
    //
    // GetCellUnionBound() implementations should attempt to return a small
    // covering (ideally 4 cells or fewer) that covers the region and can be
    // computed quickly.  The result is used by S2RegionCoverer as a starting
    // point for further refinement.
    //
    // `GetCapBound().GetCellUnionBound(cell_ids)` and
    // `GetRectBound().GetCellUnionBound(cell_ids)` are always valid
    // implementations, but something better should be done if possible.
    fn get_cell_union_bound(&self, cell_ids: &[S2CellId]);

    // Returns true if the region completely contains the given cell, otherwise
    // either the region does not contain the cell or the containment relationship
    // could not be determined.
    fn contains_cell(&self, cell: &S2Cell) -> Option<bool>;

    // Returns true if and only if the given point is contained by the region.
    // The point 'p' is generally required to be unit length, although some
    // subtypes may relax this restriction.
    fn contains_point(&self, point: &S2Point) -> bool;
}
