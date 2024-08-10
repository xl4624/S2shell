// An S2Region represents a two-dimensional region over the unit sphere.
// It is an abstract interface with various concrete subtypes.
//
// The main purpose of this interface is to allow complex regions to be
// approximated as simpler regions.  So rather than having a wide variety
// of virtual methods that are implemented by all subtypes, the interface
// is restricted to methods that are useful for computing approximations.
#[derive(Debug)]
pub struct S2Region {
}
