use crate::s2::s2cap::S2Cap;

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
}
