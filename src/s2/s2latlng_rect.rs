// An S2LatLngRect represents a closed latitude-longitude rectangle. It is
// capable of representing the empty and full rectangles as well as single
// points. Note that the latitude-longitude space is considered to have a
// *cylindrical* topology rather than a spherical one, i.e. the poles have
// multiple lat/lng representations. An S2LatLngRect may be defined so that
// includes some representations of a pole but not others. Use the
// PolarClosure() method if you want to expand a rectangle so that it contains
// all possible representations of any contained poles.
//
// Because S2LatLngRect uses S1Interval to store the longitude range,
// longitudes of -180 degrees are treated specially. Except for empty
// and full longitude spans, -180 degree longitudes will turn into +180
// degrees. This sign flip causes lng_lo() to be greater than lng_hi(),
// indicating that the rectangle will wrap around through -180 instead of
// through +179. Thus the math is consistent within the library, but the sign
// flip can be surprising, especially when working with map projections where
// -180 and +180 are at opposite ends of the flattened map. See the comments
// on S1Interval for more details.
pub struct S2LatLngRect {}
