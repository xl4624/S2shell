use std::f64::consts::PI;

use crate::s2::s2point::S2Point;

// This class represents a one-dimensional angle (as opposed to a
// two-dimensional solid angle).  It has methods for converting angles to
// or from radians, degrees, and the E5/E6/E7 representations (i.e. degrees
// multiplied by 1e5/1e6/1e7 and rounded to the nearest integer).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct S1Angle {
    radians: f64,
}

impl S1Angle {
    /// Creates an `S1Angle` from a value in radians.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::s2::s1angle::S1Angle;
    /// use std::f64::consts::PI;
    ///
    /// let angle = S1Angle::from_radians(PI / 2.0);
    /// assert_eq!(angle.radians(), PI / 2.0);
    /// ```
    pub fn from_radians(radians: f64) -> S1Angle {
        S1Angle { radians }
    }

    /// Creates an `S1Angle` from a value in degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::s2::s1angle::S1Angle;
    /// use std::f64::consts::PI;
    ///
    /// let angle = S1Angle::from_degrees(90.0);
    /// assert_eq!(angle.radians(), PI / 2.0);
    /// ```
    pub fn from_degrees(degrees: f64) -> S1Angle {
        S1Angle::from_radians(degrees * PI / 180.0)
    }

    pub fn from_points(x: S2Point, y: S2Point) -> S1Angle {
        S1Angle::from_radians(x.angle(y))
    }

    pub fn zero() -> S1Angle {
        S1Angle::from_radians(0.0)
    }

    pub fn infinity() -> S1Angle {
        S1Angle::from_radians(f64::INFINITY)
    }

    pub fn radians(&self) -> f64 {
        self.radians
    }

    pub fn degrees(&self) -> f64 {
        (180.0 / PI) * self.radians
    }

    pub fn abs(self) -> S1Angle {
        S1Angle::from_radians(self.radians.abs())
    }

    /// Normalizes this angle to the range (-180, 180] degrees.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::s2::s1angle::S1Angle;
    ///
    /// let angle = S1Angle::from_degrees(270.0);
    /// assert_eq!(angle.normalize().degrees(), -90.0);
    ///
    /// let angle = S1Angle::from_degrees(-270.0);
    /// assert_eq!(angle.normalize().degrees(), 90.0);
    ///
    /// let angle = S1Angle::from_degrees(180.0);
    /// assert_eq!(angle.normalize().degrees(), 180.0);
    /// ```
    pub fn normalize(self) -> S1Angle {
        let mut radians = self.radians.rem_euclid(2.0 * PI);
        if radians > PI {
            radians -= 2.0 * PI;
        }
        S1Angle::from_radians(radians)
    }
}

impl Default for S1Angle {
    fn default() -> S1Angle {
        S1Angle::zero()
    }
}
