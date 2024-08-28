use std::ops::{Index, IndexMut};

use crate::util::math::Vector2;

/// An R1Interval represents a closed, bounded interval on the real line.
/// It is capable of representing the empty interval (containing no points)
/// and zero-length intervals (containing a single point).
///
/// This class is intended to be copied by value as desired.  It uses
/// the default copy constructor and assignment operator.
#[derive(Debug, Copy, Clone, Default)]
pub struct R1Interval {
    bounds: Vector2<f64>,
}

impl R1Interval {
    /// If lo > hi, the interval is empty.
    pub fn new(lo: f64, hi: f64) -> R1Interval {
        R1Interval {
            bounds: Vector2::new(lo, hi),
        }
    }

    pub fn from_point(p: f64) -> R1Interval {
        R1Interval::new(p, p)
    }

    pub fn from_point_pair(p1: f64, p2: f64) -> R1Interval {
        if p1 <= p2 {
            R1Interval::new(p1, p2)
        } else {
            R1Interval::new(p2, p1)
        }
    }

    /// The low bound of the interval.
    pub fn lo(&self) -> f64 {
        self.bounds[0]
    }

    /// The high bound of the interval.
    pub fn hi(&self) -> f64 {
        self.bounds[1]
    }

    pub fn bounds(&self) -> &Vector2<f64> {
        &self.bounds
    }

    pub fn bounds_mut(&mut self) -> &mut Vector2<f64> {
        &mut self.bounds
    }

    /// Return true if the interval is empty, i.e. it contains no points.
    pub fn is_empty(&self) -> bool {
        self.lo() > self.hi()
    }

    /// Return the center of the interval.  For empty intervals,
    /// the result is arbitrary.
    pub fn get_center(&self) -> f64 {
        0.5 * (self.lo() + self.hi())
    }

    /// Return the length of the interval.  The length of an empty interval
    /// is negative.
    pub fn get_length(&self) -> f64 {
        self.hi() - self.lo()
    }

    /// Returns true if the given point is in the closed interval [lo, hi].
    pub fn contains(&self, p: f64) -> bool {
        p >= self.lo() && p <= self.hi()
    }

    /// Returns true if the given point is in the open interval (lo, hi).
    pub fn interior_contains(&self, p: f64) -> bool {
        p > self.lo() && p < self.hi()
    }

    pub fn contains_interval(y: &R1Interval) -> bool {
        todo!()
    }

    pub fn interior_contains_interval(y: &R1Interval) -> bool {
        todo!()
    }

    pub fn intersects(&self, y: &R1Interval) -> bool {
        if self.lo() <= y.lo() {
            y.lo() <= self.hi() && !y.is_empty()
        } else {
            self.lo() <= y.hi() && !self.is_empty()
        }
    }
}

impl Index<usize> for R1Interval {
    type Output = f64;

    /// The recommended style is to use `lo()` and `hi()` whenever possible,
    /// but these methods are useful when the endpoint to be selected is not constant.
    fn index(&self, index: usize) -> &Self::Output {
        &self.bounds[index]
    }
}

impl IndexMut<usize> for R1Interval {
    /// The recommended style is to use `lo()` and `hi()` whenever possible,
    /// but these methods are useful when the endpoint to be selected is not constant.
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bounds[index]
    }
}
