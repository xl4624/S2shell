use std::ops::{Add, Index, IndexMut, Mul, Sub};

use approx::{AbsDiffEq, RelativeEq};
use num_traits::{Float, Signed};

use crate::util::math::Scalar;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3<T: Scalar> {
    x: T,
    y: T,
    z: T,
}

impl<T: Scalar> Vector3<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }

    pub fn z(&self) -> T {
        self.z
    }

    /// Returns the zero `Vector3`: (0, 0, 0).
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::Vector3;
    ///
    /// let v = Vector3::zero();
    /// assert_eq!(v, Vector3::new(0.0, 0.0, 0.0));
    /// ```
    pub fn zero() -> Vector3<T> {
        Vector3::new(T::zero(), T::zero(), T::zero())
    }

    /// Dot product (scalar product) of this vector with another vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::Vector3;
    ///
    /// let v1 = Vector3::new(1.0, 2.0, 3.0);
    /// let v2 = Vector3::new(4.0, 5.0, 6.0);
    /// assert_eq!(v1.dot_prod(v2), 32.0); // 1*4 + 2*5 + 3*6 = 32
    /// ```
    pub fn dot_prod(&self, other: Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the squared Euclidean norm (the dot product with itself).
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::Vector3;
    ///
    /// let v1 = Vector3::new(1, 2, 3);
    /// assert_eq!(v1.norm2(), 14);  // 1^2 + 2^2 + 3^2 = 1 + 4 + 9
    /// ```
    ///
    /// # Note
    ///
    /// If you are comparing the magnitudes between two vectors it's more efficient to use
    /// `norm2()` rather than `norm()` to avoid having to call `sqrt()` since the square
    /// root function is mononotonically increasing for the domain of positive values.
    pub fn norm2(&self) -> T {
        self.dot_prod(*self)
    }

    /// Euclidean norm. For integer T, correct only if Norm2 does not overflow.
    pub fn norm(&self) -> f64 {
        self.norm2().to_f64().unwrap().sqrt()
    }

    /// Normalizes this vector to a unit vector (a vector with magnitude of 1).
    ///
    /// This method returns a new `Vector3<f64>` that points in the same direction as
    /// the original vector but has a magnitude (length) of 1.
    ///
    /// # Note
    ///
    /// If this method is called on a zero vector, it will return a zero vector to avoid
    /// division by zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::Vector3;
    /// use approx::assert_relative_eq;
    ///
    /// let v: Vector3<f64> = Vector3::new(3.0, 4.0, 5.0);
    /// let exp: Vector3<f64> = Vector3::new(
    ///     3.0 / v.norm(),
    ///     4.0 / v.norm(),
    ///     5.0 / v.norm(),
    /// );
    ///
    /// assert_relative_eq!(v.normalize().norm2(), 1.0);
    /// assert_relative_eq!(v.normalize(), exp);
    /// ```
    pub fn normalize(self) -> Vector3<f64> {
        let mut norm = self.norm();
        if norm != 0.0 {
            norm = 1.0 / norm;
        }
        Vector3::new(
            self.x.to_f64().unwrap() * norm,
            self.y.to_f64().unwrap() * norm,
            self.z.to_f64().unwrap() * norm,
        )
    }

    pub fn sqrt(self) -> Vector3<f64> {
        Vector3::new(
            self.x.to_f64().unwrap().sqrt(),
            self.y.to_f64().unwrap().sqrt(),
            self.z.to_f64().unwrap().sqrt(),
        )
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
    /// assert_eq!(v1.cross_prod(v2), Vector3::new(-3, 6, -3));
    /// ```
    pub fn cross_prod(&self, other: Vector3<T>) -> Vector3<T> {
        return Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        );
    }

    pub fn angle(&self, other: Vector3<T>) -> f64 {
        f64::atan2(
            self.cross_prod(other).norm(),
            self.dot_prod(other).to_f64().unwrap(),
        )
    }
}

impl<T: Scalar> Vector3<T>
where
    T: Signed,
{
    /// Unit vector orthogonal to this vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use s2shell::util::math::Vector3;
    ///
    /// let v = Vector3::new(3.0, -4.0, 3.0);
    ///
    /// // Check properties of orthogonal unit vectors
    /// assert_eq!(v.ortho().norm(), 1.0);
    /// assert_eq!(v.dot_prod(v.ortho()), 0.0);
    /// ```
    pub fn ortho(&self) -> Vector3<f64> {
        let k = self.largest_abs_component() - 1;
        let k = if k == 0 { 2 } else { k - 1 };

        let mut temp: Vector3<T> = Vector3::zero();
        temp[k as usize] = T::one();

        self.cross_prod(temp).normalize()
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
    pub fn abs(&self) -> Vector3<T> {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
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
    pub fn largest_abs_component(&self) -> i8 {
        let temp: Vector3<T> = self.abs();
        if temp.x > temp.y {
            if temp.x > temp.z {
                0
            } else {
                2
            }
        } else {
            if temp.y > temp.z {
                1
            } else {
                2
            }
        }
    }
}

impl<T: Scalar> Add for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Scalar> Sub for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Scalar> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, scalar: T) -> Self::Output {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

macro_rules! impl_scalar_mul {
    ($($ty:ident),*) => {
        $(
            impl Mul<Vector3<$ty>> for $ty {
                type Output = Vector3<$ty>;
                fn mul(self, rhs: Vector3<$ty>) -> Self::Output {
                    Vector3::new(self * rhs.x, self * rhs.y, self * rhs.z)
                }
            }
        )*
    };
}

impl_scalar_mul!(f32, f64, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl<T: Scalar> Index<usize> for Vector3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vector3: Index {index} out of bounds"),
        }
    }
}

impl<T: Scalar> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vector3: Index {index} out of bounds"),
        }
    }
}

impl<T> AbsDiffEq for Vector3<T>
where
    T: Scalar + AbsDiffEq,
    T::Epsilon: Copy,
{
    type Epsilon = T::Epsilon;

    fn default_epsilon() -> Self::Epsilon {
        T::default_epsilon()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        T::abs_diff_eq(&self.x, &other.x, epsilon)
            && T::abs_diff_eq(&self.y, &other.y, epsilon)
            && T::abs_diff_eq(&self.z, &other.z, epsilon)
    }
}

impl<T> RelativeEq for Vector3<T>
where
    T: Scalar + Float + RelativeEq,
    T::Epsilon: Copy,
{
    fn default_max_relative() -> Self::Epsilon {
        T::default_max_relative()
    }

    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        T::relative_eq(&self.x, &other.x, epsilon, max_relative)
            && T::relative_eq(&self.y, &other.y, epsilon, max_relative)
            && T::relative_eq(&self.z, &other.z, epsilon, max_relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1, 2, 3);
        let v2 = Vector3::new(4, 5, 6);
        assert_eq!(v1 + v2, Vector3::new(5, 7, 9));
        assert_eq!(v2 + v1, Vector3::new(5, 7, 9));
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(4, 5, 6);
        let v2 = Vector3::new(1, 2, 3);
        assert_eq!(v1 - v2, Vector3::new(3, 3, 3));
    }

    #[test]
    fn test_vector3_mul() {
        let v = Vector3::new(2, 3, 4);
        let s = 3;
        assert_eq!(v * s, Vector3::new(6, 9, 12));
        assert_eq!(s * v, Vector3::new(6, 9, 12));
    }
}
