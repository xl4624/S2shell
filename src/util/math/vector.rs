use internal_vector::BasicVector;
use num_traits::{Float, Signed, Zero};
use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, Div, Index, IndexMut, Mul, Sub},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3<T> {
    data: BasicVector<T>,
}

#[rustfmt::skip]
impl<T> Vector3<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T, z: T) -> Self {
        Vector3 { data: BasicVector::new(vec![x, y, z]) }
    }

    pub fn data(&self) -> &BasicVector<T> { &self.data }
    pub fn data_mut(&mut self) -> &mut BasicVector<T> { &mut self.data }
    pub fn x(&self) -> T { self.data[0] }
    pub fn y(&self) -> T { self.data[1] }
    pub fn z(&self) -> T { self.data[2] }
    pub fn set_x(&mut self, v: T) { self.data[0] = v; }
    pub fn set_y(&mut self, v: T) { self.data[1] = v; }
    pub fn set_z(&mut self, v: T) { self.data[2] = v; }
}

impl<T> Vector3<T>
where
    T: Copy + PartialOrd + Default + Signed + Zero + Sum,
    f64: From<T>,
{
    /// Returns true if this vector's dimensions are at most margin from the other vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::vector::Vector3;
    ///
    /// let v1 = Vector3::new(1.0, 2.0, 3.0);
    /// let v2 = Vector3::new(1.1, 2.1, 3.1);
    ///
    /// assert!(v1.aequal(&v2, 0.15));
    /// assert!(!v1.aequal(&v2, 0.05));
    /// ```
    pub fn aequal(&self, other: &Self, margin: f64) -> bool {
        f64::from((self.x() - other.x()).abs()) <= margin
            && f64::from((self.y() - other.y()).abs()) <= margin
            && f64::from((self.z() - other.z()).abs()) <= margin
    }

    /// Cross product of this vector with another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::vector::Vector3;
    ///
    /// let v1 = Vector3::new(1, 2, 3);
    /// let v2 = Vector3::new(4, 5, 6);
    ///
    /// assert_eq!(v1.cross_prod(&v2), Vector3::new(-3, 6, -3));
    /// ```
    pub fn cross_prod(&self, other: &Self) -> Self {
        Self::new(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    /// Unit vector orthogonal to this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::{Vector3, VecExt};
    ///
    /// let v = Vector3::new(3.0, -4.0, 3.0);
    /// // Check properties of orthogonal unit vectors
    /// assert_eq!(v.ortho().norm(), 1.0);
    /// assert_eq!(v.dot_prod(&v.ortho()), 0.0);
    /// ```
    pub fn ortho(&self) -> Self
    where
        T: Float,
    {
        let k = self.largest_abs_component();
        let k = if k == 0 { 2 } else { k - 1 };

        let mut temp: Vector3<T> = Default::default();
        temp[k as usize] = T::one();

        self.cross_prod(&temp).normalize()
    }

    /// Vector with the absolute values of each component.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::Vector3;
    ///
    /// let v1 = Vector3::new(-3.0, 2.0, 1.0);
    /// let exp1 = Vector3::new(3.0, 2.0, 1.0);
    /// assert_eq!(v1.abs(), exp1);
    ///
    /// let v2 = Vector3::new(-1, -2, -3);
    /// let exp2 = Vector3::new(1, 2, 3);
    /// assert_eq!(v2.abs(), exp2);
    /// ```
    pub fn abs(&self) -> Self {
        Vector3::new(self.x().abs(), self.y().abs(), self.z().abs())
    }

    /// Index of the component with the largest absolute value.
    ///
    /// 0 for x, 1 for y, 2 for z.
    ///
    /// # Examples
    /// ```
    /// use s2shell::util::math::Vector3;
    ///
    /// let v1 = Vector3::new(-3.0, 2.0, -1.0);
    /// assert_eq!(v1.largest_abs_component(), 0);
    ///
    /// let v2 = Vector3::new(1.0, 3.0, -2.0);
    /// assert_eq!(v2.largest_abs_component(), 1);
    ///
    /// let v3 = Vector3::new(0.0, 1.0, -2.0);
    /// assert_eq!(v3.largest_abs_component(), 2);
    /// ```
    #[rustfmt::skip]
    pub fn largest_abs_component(&self) -> i32 {
        let abs: Vector3<T> = self.abs();
        if abs.x() > abs.y() {
            if abs.x() > abs.z() { 0 } else { 2 }
        } else {
            if abs.y() > abs.z() { 1 } else { 2 }
        }
    }
}

pub trait VecExt {
    type Item: Copy + PartialOrd + Signed + Zero + Sum;

    fn dot_prod(&self, other: &Self) -> Self::Item;
    fn norm2(&self) -> Self::Item;
    fn norm(&self) -> f64;
    fn normalize(&self) -> Self
    where
        Self::Item: Float;
}

impl<T> VecExt for Vector3<T>
where
    T: Copy + PartialOrd + Signed + Zero + Sum + Default,
    f64: From<T>,
{
    type Item = T;

    /// Dot product (scalar product) of this vector with another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::{Vector3, VecExt};
    ///
    /// let v1 = Vector3::new(1.0, 2.0, 3.0);
    /// let v2 = Vector3::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(v1.dot_prod(&v2), 32.0); // 1*4 + 2*5 + 3*6 = 32
    /// ```
    fn dot_prod(&self, other: &Self) -> Self::Item {
        self.data.dot_prod(&other.data)
    }

    /// Squared Euclidean norm (the dot product with itself).
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::{Vector3, VecExt};
    ///
    /// let v1 = Vector3::new(1, 2, 3);
    /// assert_eq!(v1.norm2(), 14);  // 1^2 + 2^2 + 3^2 = 1 + 4 + 9 = 14
    /// ```
    fn norm2(&self) -> Self::Item {
        self.data.norm2()
    }

    /// Euclidean norm. For integer T, correct only if Norm2 does not overflow.
    fn norm(&self) -> f64 {
        self.data.norm()
    }

    fn normalize(&self) -> Self
    where
        T: Float,
    {
        Vector3 {
            data: self.data.normalize(),
        }
    }
}

impl<T> Add for Vector3<T>
where
    T: Copy + Default + Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vector3::from(self.data + other.data)
    }
}

impl<T> Add for &Vector3<T>
where
    T: Copy + Default + Add<Output = T>,
{
    type Output = Vector3<T>;

    fn add(self, other: Self) -> Self::Output {
        Vector3::from(&self.data + &other.data)
    }
}

impl<T> Sub for Vector3<T>
where
    T: Copy + Default + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vector3::from(self.data - other.data)
    }
}

impl<T> Sub for &Vector3<T>
where
    T: Copy + Default + Sub<Output = T>,
{
    type Output = Vector3<T>;

    fn sub(self, other: Self) -> Self::Output {
        Vector3::from(&self.data - &other.data)
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Copy + Default + ScalarOperand + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Vector3::from(self.data * scalar)
    }
}

impl<T> Mul<T> for &Vector3<T>
where
    T: Copy + Default + ScalarOperand + Mul<Output = T>,
{
    type Output = Vector3<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Vector3::from(&self.data * scalar)
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Copy + Default + Div<Output = T>,
{
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Vector3::from(self.data / scalar)
    }
}

impl<T> Div<T> for &Vector3<T>
where
    T: Copy + Default + Div<Output = T>,
{
    type Output = Vector3<T>;

    fn div(self, scalar: T) -> Self::Output {
        Vector3::from(&self.data / scalar)
    }
}

impl<T> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> Default for Vector3<T>
where
    T: Default,
{
    fn default() -> Self {
        Vector3 {
            data: BasicVector::new(vec![T::default(), T::default(), T::default()]),
        }
    }
}

pub trait ScalarOperand: Clone {}
impl ScalarOperand for i8 {}
impl ScalarOperand for u8 {}
impl ScalarOperand for i16 {}
impl ScalarOperand for u16 {}
impl ScalarOperand for i32 {}
impl ScalarOperand for u32 {}
impl ScalarOperand for i64 {}
impl ScalarOperand for u64 {}
impl ScalarOperand for i128 {}
impl ScalarOperand for u128 {}
impl ScalarOperand for isize {}
impl ScalarOperand for usize {}
impl ScalarOperand for f32 {}
impl ScalarOperand for f64 {}

mod internal_vector {
    use std::{
        iter::Sum,
        ops::{Add, Div, Index, IndexMut, Mul, Sub},
    };

    use num_traits::{Float, Signed, Zero};

    use super::{ScalarOperand, VecExt, Vector3};

    #[derive(Debug, Clone, PartialEq)]
    pub struct BasicVector<T>(Vec<T>);

    impl<T> From<BasicVector<T>> for Vector3<T>
    where
        T: Copy + Default,
    {
        fn from(vec: BasicVector<T>) -> Self {
            let mut iter = vec.into_iter();
            Vector3::new(
                iter.next().unwrap_or_default(),
                iter.next().unwrap_or_default(),
                iter.next().unwrap_or_default(),
            )
        }
    }

    impl<T> BasicVector<T> {
        pub fn new(vec: Vec<T>) -> Self {
            BasicVector(vec)
        }

        pub fn inner(&self) -> &Vec<T> {
            &self.0
        }

        pub fn inner_mut(&mut self) -> &mut Vec<T> {
            &mut self.0
        }

        pub fn into_inner(self) -> Vec<T> {
            self.0
        }

        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.inner().iter()
        }
    }

    impl<T> VecExt for BasicVector<T>
    where
        T: Copy + PartialOrd + Signed + Zero + Sum,
        f64: From<T>,
    {
        type Item = T;

        fn dot_prod(&self, other: &Self) -> Self::Item {
            self.iter().zip(other.iter()).map(|(&a, &b)| a * b).sum()
        }

        fn norm2(&self) -> Self::Item {
            self.dot_prod(self)
        }

        fn norm(&self) -> f64 {
            f64::from(self.norm2()).sqrt()
        }

        fn normalize(&self) -> Self
        where
            T: Float,
        {
            let mut norm = Self::Item::from(self.norm()).unwrap_or(T::zero());
            if norm != T::zero() {
                norm = T::one() / norm;
            }
            self.iter().map(|&x| x * norm).collect()
        }
    }

    impl<T> Add for BasicVector<T>
    where
        T: Copy + Add<Output = T>,
    {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            BasicVector(
                self.iter()
                    .zip(other.iter())
                    .map(|(&a, &b)| a + b)
                    .collect(),
            )
        }
    }

    impl<T> Add for &BasicVector<T>
    where
        T: Copy + Add<Output = T>,
    {
        type Output = BasicVector<T>;

        fn add(self, other: Self) -> Self::Output {
            BasicVector(
                self.iter()
                    .zip(other.iter())
                    .map(|(&a, &b)| a + b)
                    .collect(),
            )
        }
    }

    impl<T> Sub for BasicVector<T>
    where
        T: Copy + Sub<Output = T>,
    {
        type Output = BasicVector<T>;

        fn sub(self, other: Self) -> Self::Output {
            BasicVector(
                self.iter()
                    .zip(other.iter())
                    .map(|(&a, &b)| a - b)
                    .collect(),
            )
        }
    }

    impl<T> Sub for &BasicVector<T>
    where
        T: Copy + Sub<Output = T>,
    {
        type Output = BasicVector<T>;

        fn sub(self, other: Self) -> Self::Output {
            BasicVector(
                self.iter()
                    .zip(other.iter())
                    .map(|(&a, &b)| a - b)
                    .collect(),
            )
        }
    }

    impl<T> Mul<T> for BasicVector<T>
    where
        T: Copy + ScalarOperand + Mul<Output = T>,
    {
        type Output = BasicVector<T>;

        fn mul(self, scalar: T) -> Self::Output {
            BasicVector(self.iter().map(|&x| x * scalar).collect())
        }
    }

    impl<T> Mul<T> for &BasicVector<T>
    where
        T: Copy + ScalarOperand + Mul<Output = T>,
    {
        type Output = BasicVector<T>;

        fn mul(self, scalar: T) -> Self::Output {
            BasicVector(self.iter().map(|&x| x * scalar).collect())
        }
    }

    impl<T> Div<T> for BasicVector<T>
    where
        T: Copy + Div<Output = T>,
    {
        type Output = BasicVector<T>;

        fn div(self, scalar: T) -> Self::Output {
            BasicVector(self.iter().map(|&x| x / scalar).collect())
        }
    }

    impl<T> Div<T> for &BasicVector<T>
    where
        T: Copy + Div<Output = T>,
    {
        type Output = BasicVector<T>;

        fn div(self, scalar: T) -> Self::Output {
            BasicVector(self.iter().map(|&x| x / scalar).collect())
        }
    }

    impl<T> IntoIterator for BasicVector<T> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            self.into_inner().into_iter()
        }
    }

    impl<T> FromIterator<T> for BasicVector<T> {
        fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
            BasicVector(Vec::from_iter(iter))
        }
    }

    impl<T> Index<usize> for BasicVector<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            &self.inner()[index]
        }
    }

    impl<T> IndexMut<usize> for BasicVector<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.inner_mut()[index]
        }
    }

    macro_rules! impl_scalar_op {
    ($scalar:ty, $op:tt, $trait:ident, $method:ident) => {
        impl $trait<Vector3<$scalar>> for $scalar {
            type Output = Vector3<$scalar>;

            fn $method(self, rhs: Vector3<$scalar>) -> Self::Output {
                Vector3::from(self $op rhs.data)
            }
        }

        impl $trait<&Vector3<$scalar>> for $scalar {
            type Output = Vector3<$scalar>;

            fn $method(self, rhs: &Vector3<$scalar>) -> Self::Output {
                Vector3::from(self $op &rhs.data)
            }
        }

        impl $trait<BasicVector<$scalar>> for $scalar {
            type Output = BasicVector<$scalar>;

            fn $method(self, rhs: BasicVector<$scalar>) -> Self::Output {
                BasicVector(rhs.iter().map(|&x| self $op x).collect())
            }
        }

        impl $trait<&BasicVector<$scalar>> for $scalar {
            type Output = BasicVector<$scalar>;

            fn $method(self, rhs: &BasicVector<$scalar>) -> Self::Output {
                BasicVector(rhs.iter().map(|&x| self $op x).collect())
            }
        }
    }
}

    macro_rules! impl_all_scalar_ops {
    ($($t:ty),*) => {
        $(
            impl_scalar_op!($t, +, Add, add);
            impl_scalar_op!($t, -, Sub, sub);
            impl_scalar_op!($t, *, Mul, mul);
            impl_scalar_op!($t, /, Div, div);
        )*
    }
}

    impl_all_scalar_ops!(i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize, f32, f64);
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(4, 5, 6);
        assert_eq!(&v1 + &v2, Vector3::new(5, 7, 9));
        assert_eq!(&v2 + &v1, Vector3::new(5, 7, 9));
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(4, 5, 6);
        let v2 = Vector3::new(1, 2, 3);
        assert_eq!(&v1 - &v2, Vector3::new(3, 3, 3));
        assert_eq!(v1 - v2, Vector3::new(3, 3, 3));
    }

    #[test]
    fn test_vector3_mul() {
        let v = Vector3::new(2, 3, 4);
        let s = 3;
        assert_eq!(&v * s, Vector3::new(6, 9, 12));
        assert_eq!(s * &v, Vector3::new(6, 9, 12));
    }

    #[test]
    fn test_vector3_div() {
        let v = Vector3::new(6, 9, 12);
        let s = 3;
        assert_eq!(v / s, Vector3::new(2, 3, 4));
    }
}
