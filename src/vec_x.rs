use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use num::Num;

/// A structure representing a fixed-length array of arbitrary elements and arbitrary length.
/// Since it was created primarily to represent mathematical vectors and colors, it supports four arithmetic operations.
///
/// 任意の要素、任意の長さの固定長配列を表す構造体です。
/// 主に数学的なベクトルや色を表すために作成したため、四則演算をサポートしています。
///
///
///
/// ```
/// use vec_x::{VecX};
///
/// let vec1 = VecX::new([1, 2, 3]);
/// let vec2 = VecX::new([4, 5, 6]);
///
/// // Add
/// assert_eq!(vec1 + vec2, VecX::new([5, 7, 9]));
/// // Sub
/// assert_eq!(vec1 - vec2, VecX::new([-3, -3, -3]));
/// // Mul
/// assert_eq!(vec1 * vec2, VecX::new([4, 10, 18]));
/// // Div
/// assert_eq!(vec1 / vec2, VecX::new([0, 0, 0]));
/// // Rem
/// assert_eq!(vec1 % vec2, VecX::new([1, 2, 3]));
///
/// // AddAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec += VecX::new([4, 5, 6]);
/// assert_eq!(vec, VecX::new([5, 7, 9]));
/// // SubAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec -= VecX::new([4, 5, 6]);
/// assert_eq!(vec, VecX::new([-3, -3, -3]));
/// // MulAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec *= VecX::new([4, 5, 6]);
/// assert_eq!(vec, VecX::new([4, 10, 18]));
/// // DivAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec /= VecX::new([4, 5, 6]);
/// assert_eq!(vec, VecX::new([0, 0, 0]));
/// // RemAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec %= VecX::new([4, 5, 6]);
/// assert_eq!(vec, VecX::new([1, 2, 3]));
/// ```
///
/// Non-numeric elements can also be array elements.
///
/// 数値以外を配列要素にすることもできます。
///
/// ```
/// use vec_x::{VecX};
///
/// let vec1 = VecX::new(["a", "b", "c"]);
/// ```
///
/// ```compile_fail
/// use vec_x::{VecX};
///
///
/// let vec1 = VecX::new(["a", "b", "c"]);
/// let vec2 = VecX::new(["d", "e", "f"]);
///
/// vec1 + vec2; // compile error!
/// ```
///
/// Using type aliases, as shown below, improves code readability.
///
/// 以下のように型エイリアスを使用することで、コードの可読性が向上します。
///
/// ```
/// use vec_x::{VecX};
///
/// type XYZ = VecX<f64, 3>;
/// type RGBA = VecX<u8, 4>;
///
/// struct Point {
///    position: XYZ,
///    color: RGBA,
/// }
///
/// let point = Point {
///    position: XYZ::new([1.0, 2.0, 3.0]),
///    color: RGBA::new([255, 0, 0, 255]),
/// };
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct VecX<T, const N: usize> {
    pub data: [T; N],
}

impl<T: Default + Copy, const N: usize> Default for VecX<T, N> {
    fn default() -> Self {
        Self { data: [T::default(); N] }
    }
}

impl<T, const N: usize> VecX<T, N> {
    /// Generate a new `VecX`.
    ///
    /// 新しい `VecX` を生成する。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec = VecX::new([1, 2, 3]);
    /// ```
    pub fn new(data: [T; N]) -> Self {
        Self { data }
    }
}

impl<T, const N: usize> Index<usize> for VecX<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const N: usize> IndexMut<usize> for VecX<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T: Num + Copy, const N: usize> Add<Self> for VecX<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let data: [T; N] = self.data.into_iter().zip(rhs.data).map(|(a, b)| a + b).collect::<Vec<_>>().as_slice().try_into().unwrap();

        Self { data }
    }
}

impl<T: Num + Copy, const N: usize> Sub<Self> for VecX<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let data: [T; N] = self.data.into_iter().zip(rhs.data).map(|(a, b)| a - b).collect::<Vec<_>>().as_slice().try_into().unwrap();

        Self { data }
    }
}

impl<T: Num + Copy, const N: usize> Mul<Self> for VecX<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let data: [T; N] = self.data.into_iter().zip(rhs.data).map(|(a, b)| a * b).collect::<Vec<_>>().as_slice().try_into().unwrap();

        Self { data }
    }
}

impl<T: Num + Copy, const N: usize> Div<Self> for VecX<T, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let data: [T; N] = self.data.into_iter().zip(rhs.data).map(|(a, b)| a / b).collect::<Vec<_>>().as_slice().try_into().unwrap();

        Self { data }
    }
}

impl<T: Num + Copy, const N: usize> Rem<Self> for VecX<T, N> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let data: [T; N] = self.data.into_iter().zip(rhs.data).map(|(a, b)| a % b).collect::<Vec<_>>().as_slice().try_into().unwrap();

        Self { data }
    }
}

impl<T: Num + AddAssign, const N: usize> AddAssign for VecX<T, N> {
    fn add_assign(&mut self, rhs: Self) {
        rhs.data.into_iter().enumerate().for_each(|(i, v)| self.data[i] += v);
    }
}

impl<T: Num + SubAssign, const N: usize> SubAssign for VecX<T, N> {
    fn sub_assign(&mut self, rhs: Self) {
        rhs.data.into_iter().enumerate().for_each(|(i, v)| self.data[i] -= v);
    }
}

impl<T: Num + MulAssign, const N: usize> MulAssign for VecX<T, N> {
    fn mul_assign(&mut self, rhs: Self) {
        rhs.data.into_iter().enumerate().for_each(|(i, v)| self.data[i] *= v);
    }
}

impl<T: Num + DivAssign, const N: usize> DivAssign for VecX<T, N> {
    fn div_assign(&mut self, rhs: Self) {
        rhs.data.into_iter().enumerate().for_each(|(i, v)| self.data[i] /= v);
    }
}

impl<T: Num + RemAssign, const N: usize> RemAssign for VecX<T, N> {
    fn rem_assign(&mut self, rhs: Self) {
        rhs.data.into_iter().enumerate().for_each(|(i, v)| self.data[i] %= v);
    }
}
