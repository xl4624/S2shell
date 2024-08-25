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

/// An S2LatLngRect represents a closed latitude-longitude rectangle. It is
/// capable of representing the empty and full rectangles as well as single
/// points. Note that the latitude-longitude space is considered to have a
/// *cylindrical* topology rather than a spherical one, i.e. the poles have
/// multiple lat/lng representations. An S2LatLngRect may be defined so that
/// includes some representations of a pole but not others. Use the
/// PolarClosure() method if you want to expand a rectangle so that it contains
/// all possible representations of any contained poles.
///
/// Because S2LatLngRect uses S1Interval to store the longitude range,
/// longitudes of -180 degrees are treated specially. Except for empty
/// and full longitude spans, -180 degree longitudes will turn into +180
/// degrees. This sign flip causes lng_lo() to be greater than lng_hi(),
/// indicating that the rectangle will wrap around through -180 instead of
/// through +179. Thus the math is consistent within the library, but the sign
/// flip can be surprising, especially when working with map projections where
/// -180 and +180 are at opposite ends of the flattened map. See the comments
/// on S1Interval for more details.
#[derive(Debug)]
pub struct S2LatLngRect {}
