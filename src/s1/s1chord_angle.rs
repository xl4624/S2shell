use crate::{s1::S1Angle, s2::S2Point};

// S1ChordAngle represents the angle subtended by a chord (i.e., the straight
// line segment connecting two points on the sphere). Its representation
// makes it very efficient for computing and comparing distances, but unlike
// S1Angle it is only capable of representing angles between 0 and Pi radians.
// S1ChordAngle is intended for applications where many angles need to be
// computed and compared, otherwise it is simpler to use S1Angle.
//
// S1ChordAngle also loses some accuracy as the angle approaches Pi radians.
// There are several different ways to measure this error, including the
// representational error (i.e., how accurately S1ChordAngle can represent
// angles near Pi radians), the conversion error (i.e., how much precision is
// lost when an S1Angle is converted to an S1ChordAngle), and the measurement
// error (i.e., how accurate the S1ChordAngle(a, b) constructor is when the
// points A and B are separated by angles close to Pi radians). All of these
// errors differ by a small constant factor.
//
// For the measurement error (which is the largest of these errors and also
// the most important in practice), let the angle between A and B be (Pi - x)
// radians, i.e. A and B are within "x" radians of being antipodal. The
// corresponding chord length is
//
//    r = 2 * sin((Pi - x) / 2) = 2 * cos(x / 2) .
//
// For values of x not close to Pi the relative error in the squared chord
// length is at most 4.5 * DBL_EPSILON (see GetS2PointConstructorMaxError).
// The relative error in "r" is thus at most 2.25 * DBL_EPSILON ~= 5e-16. To
// convert this error into an equivalent angle, we have
//
//    |dr / dx| = sin(x / 2)
//
// and therefore
//
//    |dx| = dr / sin(x / 2)
//         = 5e-16 * (2 * cos(x / 2)) / sin(x / 2)
//         = 1e-15 / tan(x / 2)
//
// The maximum error is attained when
//
//    x  = |dx|
//       = 1e-15 / tan(x / 2)
//      ~= 1e-15 / (x / 2)
//      ~= sqrt(2e-15)
//
// In summary, the measurement error for an angle (Pi - x) is at most
//
//    dx  = min(1e-15 / tan(x / 2), sqrt(2e-15))
//      (~= min(2e-15 / x, sqrt(2e-15)) when x is small).
//
// On the Earth's surface (assuming a radius of 6371km), this corresponds to
// the following worst-case measurement errors:
//
//     Accuracy:             Unless antipodal to within:
//     ---------             ---------------------------
//     6.4 nanometers        10,000 km (90 degrees)
//     1 micrometer          81.2 kilometers
//     1 millimeter          81.2 meters
//     1 centimeter          8.12 meters
//     28.5 centimeters      28.5 centimeters
//
// The representational and conversion errors referred to earlier are somewhat
// smaller than this. For example, maximum distance between adjacent
// representable S1ChordAngle values is only 13.5 cm rather than 28.5 cm. To
// see this, observe that the closest representable value to r^2 = 4 is
// r^2 =  4 * (1 - DBL_EPSILON / 2). Thus r = 2 * (1 - DBL_EPSILON / 4) and
// the angle between these two representable values is
//
//    x  = 2 * acos(r / 2)
//       = 2 * acos(1 - DBL_EPSILON / 4)
//      ~= 2 * asin(sqrt(DBL_EPSILON / 2)
//      ~= sqrt(2 * DBL_EPSILON)
//      ~= 2.1e-8
//
// which is 13.5 cm on the Earth's surface.
//
// The worst case rounding error occurs when the value halfway between these
// two representable values is rounded up to 4. This halfway value is
// r^2 = (4 * (1 - DBL_EPSILON / 4)), thus r = 2 * (1 - DBL_EPSILON / 8) and
// the worst case rounding error is
//
//    x  = 2 * acos(r / 2)
//       = 2 * acos(1 - DBL_EPSILON / 8)
//      ~= 2 * asin(sqrt(DBL_EPSILON / 4)
//      ~= sqrt(DBL_EPSILON)
//      ~= 1.5e-8
//
// which is 9.5 cm on the Earth's surface.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct S1ChordAngle {
    length2: f64,
}

impl S1ChordAngle {
    pub fn new(length2: f64) -> S1ChordAngle {
        S1ChordAngle { length2 }
    }

    pub fn from_points(x: &S2Point, y: &S2Point) -> S1ChordAngle {
        todo!()
    }

    pub fn zero() -> S1ChordAngle {
        S1ChordAngle::new(0.0)
    }

    /// Return a chord angle of 90 degrees (a "right angle").
    pub fn right() -> S1ChordAngle {
        S1ChordAngle::new(2.0)
    }

    /// Return a chord angle of 180 degrees (a "straight angle"). This is the
    /// maximum finite chord angle.
    pub fn straight() -> S1ChordAngle {
        S1ChordAngle::new(4.0)
    }

    // Return a chord angle larger than any finite chord angle. The only valid
    // operations on Infinity() are comparisons, S1Angle conversions, and
    // Successor() / Predecessor().
    pub fn infinity() -> S1ChordAngle {
        S1ChordAngle::new(f64::INFINITY)
    }

    // Return a chord angle smaller than Zero(). The only valid operations on
    // Negative() are comparisons, S1Angle conversions, and Successor() /
    // Predecessor().
    pub fn negative() -> S1ChordAngle {
        S1ChordAngle::new(-1.0)
    }

    pub fn from_radians(radians: f64) -> S1ChordAngle {
        S1ChordAngle::from(S1Angle::from_radians(radians))
    }

    pub fn from_degrees(degrees: f64) -> S1ChordAngle {
        S1ChordAngle::from(S1Angle::from_degrees(degrees))
    }

    /// Construct an S1ChordAngle that is an upper bound on the given S1Angle,
    /// i.e. such that S1Angle::from(S1ChordAngle::fast_upper_bound_from(x)) >= x.
    /// Unlike the S1Angle constructor above, this method is very fast, and the
    /// bound is accurate to within 1% for distances up to about 3100 km on the
    /// Earth's surface.
    pub fn fast_upper_bound_from(angle: S1Angle) -> S1ChordAngle {
        todo!()
    }

    /// Construct an S1ChordAngle from the squared chord length. Note that the
    /// argument is automatically clamped to a maximum of 4.0 to handle possible
    /// roundoff errors. The argument must be non-negative.
    pub fn from_length2(length2: f64) -> S1ChordAngle {
        todo!()
    }

    /// Convenience methods implemented by calling S1Angle::from() first. Note that
    /// because of the S1Angle conversion these methods are relatively expensive
    /// (despite their lowercase names), so the results should be cached if they
    /// are needed inside loops.
    pub fn radians(&self) -> f64 {
        todo!()
    }

    /// Convenience methods implemented by calling S1Angle::from() first. Note that
    /// because of the S1Angle conversion these methods are relatively expensive
    /// (despite their lowercase names), so the results should be cached if they
    /// are needed inside loops.
    pub fn degrees(&self) -> f64 {
        todo!()
    }


    pub fn is_negative(&self) -> bool {
        todo!()
    }

    pub fn is_infinity(&self) -> bool {
        todo!()
    }

    pub fn length2(&self) -> f64 {
        self.length2
    }
}

impl From<S1Angle> for S1ChordAngle {
    /// Conversion from an S1Angle. Angles outside the range [0, Pi] are handled
    /// as follows: Infinity() is mapped to Infinity(), negative angles are
    /// mapped to Negative(), and finite angles larger than Pi are mapped to
    /// Straight().

    /// Note that this operation is relatively expensive and should be avoided.
    /// To use S1ChordAngle effectively, you should structure your code so that
    /// input arguments are converted to S1ChordAngles at the beginning of your
    /// algorithm, and results are converted back to S1Angles only at the end.
    fn from(value: S1Angle) -> S1ChordAngle {
        todo!()
    }
}
