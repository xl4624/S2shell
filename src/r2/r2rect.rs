use std::ops::{Index, IndexMut};

use crate::r1::R1Interval;

/// An R2Rect represents a closed axis-aligned rectangle in the (x,y) plane.
#[derive(Debug, Copy, Clone, Default)]
pub struct R2Rect {
    bounds: [R1Interval; 2],
}

impl Index<usize> for R2Rect {
    type Output = R1Interval;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bounds[index]
    }
}

impl IndexMut<usize> for R2Rect {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bounds[index]
    }
}
