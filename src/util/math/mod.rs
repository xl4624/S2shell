use num_traits::{Num, NumCast};

pub mod vector;
pub use vector::Vector3;

pub trait Scalar: Copy + Clone + Num + NumCast + PartialOrd {}
impl<T> Scalar for T where T: Copy + Clone + Num + NumCast + PartialOrd {}
