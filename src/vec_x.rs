use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use num::Num;
use num::traits::AsPrimitive;

/// A structure representing a fixed-length array of arbitrary elements and arbitrary length.
/// Since it was created primarily to represent mathematical vectors and colors, it supports four arithmetic operations.
///
/// 任意の要素、任意の長さの固定長配列を表す構造体です。
/// 主に数学的なベクトルや色を表すために作成したため、四則演算をサポートしています。
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
///　You can also perform arithmetic operations with numeric values.
///
/// 数値型の値との演算も可能です。
///
/// ```
/// use vec_x::{VecX};
///
/// let vec = VecX::new([1, 2, 3]);
///
/// // Add
/// assert_eq!(vec + 1, VecX::new([2, 3, 4]));
///
/// // Sub
/// assert_eq!(vec - 1, VecX::new([0, 1, 2]));
///
/// // Mul
/// assert_eq!(vec * 2, VecX::new([2, 4, 6]));
///
/// // Div
/// assert_eq!(vec / 2, VecX::new([0, 1, 1]));
///
/// // Rem
/// assert_eq!(vec % 2, VecX::new([1, 0, 1]));
///
/// // AddAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec += 1;
/// assert_eq!(vec, VecX::new([2, 3, 4]));
///
/// // SubAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec -= 1;
/// assert_eq!(vec, VecX::new([0, 1, 2]));
///
/// // MulAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec *= 2;
/// assert_eq!(vec, VecX::new([2, 4, 6]));
///
/// // DivAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec /= 2;
/// assert_eq!(vec, VecX::new([0, 1, 1]));
///
/// // RemAssign
/// let mut vec = VecX::new([1, 2, 3]);
/// vec %= 2;
/// assert_eq!(vec, VecX::new([1, 0, 1]));
/// ```
///
/// In operations, arrays that implement From/Into traits are implicitly converted to the left-hand side type.
///
/// 演算において、From/Intoトレイトが実装されている配列同士は暗黙的に左辺の型に変換されます。
///
/// ```
/// use vec_x::{VecX};
/// use std::ops::Add;
///
/// let vec1:VecX<f32,3> = VecX::new([1., 2., 3.]);
/// let vec2:VecX<u8,3> = VecX::new([4, 5, 6]);
///
/// let vec3 = vec1 + vec2;
/// ```
/// Arrays that do not implement From/Into trait will fail to compile together.
/// Thus, there is no loss of precision due to implicit type conversion.
///
/// From/Intoトレイトが実装されていない配列同士はコンパイルエラーになります。
/// よって、暗黙的な型変換によって精度が失われることはありません。
///
/// ```compile_fail
/// use vec_x::{VecX};
/// use std::ops::Add;
///
/// let vec1:VecX<f32,3> = VecX::new([1., 2., 3.]);
/// let vec2:VecX<u8,3> = VecX::new([4, 5, 6]);
///
/// let vec3 = vec2 + vec1; // compile error! Cannot add `VecX<f32, 3>` to `VecX<u8, 3>`[E0369]
/// ```
///
/// Element casts are also supported.
///
/// 要素のキャストにも対応しています。
///
/// ```
/// use vec_x::{VecX};
///
/// let vec = VecX::new([1, 2, 3]);
///
/// let vec_f64:VecX<f64,3> = vec.as_();
/// ```
///
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

    /// Cast `VecX<T, N>` to `VecX<U, N>`.
    /// Array elements are cast in the same way as numeric types.
    ///
    /// `VecX<T, N>`を`VecX<U, N>`にキャストする。
    /// 配列の要素は数値型と同じ方法でキャストされる。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec = VecX::new([1, 2, 3]);
    ///
    /// let vec_f64:VecX<f64,3> = vec.as_();
    /// ```
    pub fn as_<U>(&self) -> VecX<U, N>
        where
            U: AsPrimitive<T>,
            T: AsPrimitive<U>
    {
        let data: [U; N] = self.data.map(|v| v.as_());
        VecX { data }
    }

    /// Match the number of elements in `VecX<T, N>` to M.
    /// If `M > T`, empty elements are filled with the value of `T::default()`.
    ///
    /// `VecX<T, N>`の要素数をMに合わせる。
    /// `M > T`である場合、空の要素は`T::default()`の値で満たされる。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec = VecX::new([1, 2, 3]);
    /// assert_eq!(vec.fit::<2>(), VecX::new([1, 2]));
    ///
    /// let vec = VecX::new([1, 2, 3]);
    /// assert_eq!(vec.fit::<5>(), VecX::new([1, 2, 3, 0, 0]));
    /// ```
    pub fn fit<const M: usize>(&self) -> VecX<T, M>
        where T: Default + Copy {
        let mut data = [T::default(); M];

        (0..N.min(M)).for_each(|i| data[i] = self.data[i]);

        VecX { data }
    }

    /// Apply a function to each element of `VecX<T, N>`.
    ///
    /// `VecX<T, N>`の各要素に関数を適用する。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let do_something = |v:u32| v.to_string();
    ///
    /// let vec = VecX::new([1, 2, 3]);
    ///
    /// assert_eq!(vec.batch(do_something), VecX::new(["1".to_string(), "2".to_string(), "3".to_string()]));
    /// ```
    pub fn batch<U, F>(self, callback: F) -> VecX<U, N>
        where F: Fn(T) -> U
    {
        let data = self.data.map(callback);
        VecX { data }
    }

    /// Apply a function to each element of `VecX<T, N>` and `VecX<U, N>`.
    ///
    /// `VecX<T, N>`と`VecX<U, N>`の各要素に関数を適用する。
    ///
    /// # Examples
    ///
    /// ```
    /// use vec_x::{VecX};
    ///
    /// let vec1 = VecX::new([1, 5, 3]);
    /// let vec2 = VecX::new([4, 2, 6]);
    ///
    /// assert_eq!(vec1.batch_with(vec2, |a, b| a.min(b)), VecX::new([1, 2, 3])); 
    /// ```

    pub fn batch_with<U, V, F>(self, other: VecX<U, N>, callback: F) -> VecX<V, N>
        where V: Default + Copy,
              T: Copy,
              U: Copy,
              F: Fn(T, U) -> V

    {
        let mut data = [V::default(); N];

        (0..N).for_each(|i| data[i] = callback(self.data[i], other.data[i]));

        VecX { data }
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

impl<T, U, const N: usize> Add<U> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn add(mut self, rhs: U) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] + rhs.into());

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

impl<T, U, const N: usize> Sub<U> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn sub(mut self, rhs: U) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] - rhs.into());

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

impl<T, U, const N: usize> Mul<U> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn mul(mut self, rhs: U) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] * rhs.into());

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

impl<T, U, const N: usize> Div<U> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn div(mut self, rhs: U) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] / rhs.into());

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

impl<T, U, const N: usize> Rem<U> for VecX<T, N>
    where T: Num + Copy,
          U: Num + Copy + Into<T>
{
    type Output = Self;

    fn rem(mut self, rhs: U) -> Self::Output {
        (0..N).for_each(|i| self.data[i] = self.data[i] % rhs.into());

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

impl<T, U, const N: usize> AddAssign<U> for VecX<T, N>
    where T: Num + AddAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn add_assign(&mut self, rhs: U) {
        (0..N).for_each(|i| self.data[i] += rhs.into());
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

impl<T, U, const N: usize> SubAssign<U> for VecX<T, N>
    where T: Num + SubAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn sub_assign(&mut self, rhs: U) {
        (0..N).for_each(|i| self.data[i] -= rhs.into());
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

impl<T, U, const N: usize> MulAssign<U> for VecX<T, N>
    where T: Num + MulAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn mul_assign(&mut self, rhs: U) {
        (0..N).for_each(|i| self.data[i] *= rhs.into());
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

impl<T, U, const N: usize> DivAssign<U> for VecX<T, N>
    where T: Num + DivAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn div_assign(&mut self, rhs: U) {
        (0..N).for_each(|i| self.data[i] /= rhs.into());
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

impl<T, U, const N: usize> RemAssign<U> for VecX<T, N>
    where T: Num + RemAssign + Copy,
          U: Num + Copy + Into<T>
{
    fn rem_assign(&mut self, rhs: U) {
        (0..N).for_each(|i| self.data[i] %= rhs.into());
    }
}
