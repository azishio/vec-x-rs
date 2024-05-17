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

    /// Generate a `VecX` initialized with a single value.
    ///
    /// 単一の値で初期化された `VecX` を生成する。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec = VecX::new_with(1);
    ///
    /// assert_eq!(vec, VecX::new([1, 1, 1]));
    /// ```
    pub fn new_with(value: T) -> Self
        where
            T: Copy,
    {
        Self { data: [value; N] }
    }


    /// Convert `VecX<T, N>` to `VecX<U, N>`.
    ///
    /// `VecX<T, N>`を`VecX<U, N>`に変換する。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec:VecX<u8,3> = VecX::new([1, 2, 3]);
    /// let vec_f64:VecX<f64,3> = vec.into();
    /// ```
    pub fn into<U>(self) -> VecX<U, N>
        where
            T: Into<U>
    {
        let data: [U; N] = self.data.map(|v| v.into());
        VecX { data }
    }

    /// Convert `VecX<T, N>` from `VecX<U, N>`.
    ///
    /// `VecX<U, N>`から`VecX<T, N>`に変換する。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec = VecX::new([1, 2, 3]);
    /// let vec_i32:VecX<i32,3> = VecX::from(vec);
    /// ```
    pub fn from<U>(vec: VecX<U, N>) -> Self
        where
            T: From<U>
    {
        let data: [T; N] = vec.data.map(|v| T::from(v));
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

impl<T, U, const N: usize> Add<VecX<U, N>> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn add(mut self, rhs: VecX<U, N>) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] + rhs.data[i].into());

        self
    }
}

impl<T, U, const N: usize> Sub<VecX<U, N>> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn sub(mut self, rhs: VecX<U, N>) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] - rhs.data[i].into());

        self
    }
}

impl<T, U, const N: usize> Mul<VecX<U, N>> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn mul(mut self, rhs: VecX<U, N>) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] * rhs.data[i].into());

        self
    }
}


impl<T, U, const N: usize> Div<VecX<U, N>> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn div(mut self, rhs: VecX<U, N>) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] / rhs.data[i].into());

        self
    }
}

impl<T, U, const N: usize> Rem<VecX<U, N>> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn rem(mut self, rhs: VecX<U, N>) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] % rhs.data[i].into());

        self
    }
}

impl<T, U, const N: usize> AddAssign<VecX<U, N>> for VecX<T, N>
    where T: Num + AddAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn add_assign(&mut self, rhs: VecX<U, N>) {
        (0..N).for_each(|i| self.data[i] += rhs.data[i].into());
    }
}

impl<T, U, const N: usize> SubAssign<VecX<U, N>> for VecX<T, N>
    where T: Num + SubAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn sub_assign(&mut self, rhs: VecX<U, N>) {
        (0..N).for_each(|i| self.data[i] -= rhs.data[i].into());
    }
}

impl<T, U, const N: usize> MulAssign<VecX<U, N>> for VecX<T, N>
    where T: Num + MulAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn mul_assign(&mut self, rhs: VecX<U, N>) {
        (0..N).for_each(|i| self.data[i] *= rhs.data[i].into());
    }
}

impl<T, U, const N: usize> DivAssign<VecX<U, N>> for VecX<T, N>
    where T: Num + DivAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn div_assign(&mut self, rhs: VecX<U, N>) {
        (0..N).for_each(|i| self.data[i] /= rhs.data[i].into());
    }
}

impl<T, U, const N: usize> RemAssign<VecX<U, N>> for VecX<T, N>
    where T: Num + RemAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn rem_assign(&mut self, rhs: VecX<U, N>) {
        (0..N).for_each(|i| self.data[i] %= rhs.data[i].into());
    }
}
