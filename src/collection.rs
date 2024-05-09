use std::hash::Hash;
use std::ops::Index;

use fxhash::FxBuildHasher;
use indexmap::IndexSet;

use crate::VecX;

struct VecXList<T, const N: usize> {
    data: Vec<VecX<T, N>>,
}

/// インデックスが付けられた`VecX`の集合を表す構造体です。
/// 一意の`VecX`に対してインデックスを持ち、ユースケースによってはVecXの集合を効率的に扱えます。
///
/// 例えば、3D空間上の頂点軍を表す`VecX`の集合を考えた際、同じ座標を持つ頂点が複数存在する場合に有効です。
/// このデータの管理方法は、しばしば3Dデータのフォーマットに採用されています。
///
/// ```
/// use vec_x::{VecX, IndexedVecXs};
///
/// let colors: Vec<VecX<u8, 3>> = [
///     [255, 0, 0],
///     [0, 255, 0],
///     [255, 0, 0],
///     [255, 0, 0],
///     [0, 0, 255],
///     [0, 255, 0],
/// ].into_iter().map(|p| VecX::new(p)).collect::<Vec<_>>();
/// let indexed_colors = IndexedVecXs::from_vec(colors);
/// let IndexedVecXs { values, indices } = indexed_colors;
///
///
/// // インデックスされた要素の数は元データの一意な要素の数と一致する
/// assert_eq!(values.len(), 3);
/// // 元データでの出現順にインデックスが付けられている
/// assert_eq!(values[0], VecX::new([255, 0, 0]));
/// assert_eq!(values[1], VecX::new([0, 255, 0]));
/// assert_eq!(values[2], VecX::new([0, 0, 255]));
///
/// // indicesの要素数は元データの要素数と一致する
/// assert_eq!(indices.len(), 6);
/// // 同じ要素は同じインデックスを持つ
/// assert_eq!(indices, vec![0, 1, 0, 0, 2, 1]);
/// ```
///
/// また、`IndexedVecXs`は`Index`トレイトを実装しており、インデックスを使用して要素にアクセスできます。
///
/// ```
/// use vec_x::{VecX, IndexedVecXs};
///
/// let colors: Vec<VecX<u8, 3>> = [
///    [255, 0, 0],
///    [0, 255, 0],
/// ].into_iter().map(|p| VecX::new(p)).collect::<Vec<_>>();
///
/// let indexed_colors = IndexedVecXs::from_vec(colors);
///
/// assert_eq!(indexed_colors[0], VecX::new([255, 0, 0]));
/// assert_eq!(indexed_colors[1], VecX::new([0, 255, 0]));
/// ```
///
/// `VecX`の要素は`PartialEq`,`Eq`,`Hash`を実装している必要があります。
/// これは内部でHashSetを使用しているためです。
/// Rust標準の浮動小数点数型は`Eq`,`Hash`を実装していない(NaNが複数のバイナリ表現を持つことに起因する)ため、`f32`や`f64`を使用することはできません。
///
/// もし浮動小数点数を扱いたい場合、[ordered-floatクレート](https://crates.io/crates/ordered-float)を使用することを検討してください。
///
/// ```compile_fail
/// use vec_x::{VecX, IndexedVecXs};
///
/// let points: Vec<VecX<f64, 3>> = [
///    [0., 0., 0.],
///    [1., 0., 0.],
/// ].into_iter().map(|p| VecX::new(p)).collect::<Vec<_>>();
///
/// let indexed_colors = IndexedVecXs::from_vec(points); // compile error
/// ```
pub struct IndexedVecXs<T: PartialEq + Eq + Hash, const N: usize> {
    /// 一意な`VecX`の集合
    pub values: IndexSet<VecX<T, N>, FxBuildHasher>,
    /// `values`を参照するインデックス
    pub indices: Vec<usize>,
}

impl<T: PartialEq + Eq + Hash, const N: usize> IndexedVecXs<T, N> {
    /// これは通常使用されません。`Vec<VecX<T, N>>`から`IndexedVecXs`を生成するためには`from_vec`を使用してください。
    pub fn new(
        values: IndexSet<VecX<T, N>, FxBuildHasher>,
        indices: Vec<usize>) -> Self {
        Self {
            values,
            indices,
        }
    }

    /// 空の`IndexedVecXs`を生成します。
    pub fn empty() -> Self {
        Self {
            values: IndexSet::<VecX<T, N>, FxBuildHasher>::default(),
            indices: Vec::new(),
        }
    }

    /// イテレーション可能な構造体`IndexedVecXIter`を返します。
    /// 内部的には、イテレータが消費されるたびに`values`から`indices`中のインデックスに対応する`VecX`を検索しています。
    pub fn iter(&self) -> Vec<&VecX<T, N>> {
        self.indices.iter().map(|i| self.values.get_index(*i).unwrap()).collect::<Vec<_>>()
    }

    /// `Vec<VecX<T, N>>`から`IndexedVecXs`を生成します。
    pub fn from_vec(vec: Vec<VecX<T, N>>) -> Self {
        let mut values = IndexSet::<VecX<T, N>, FxBuildHasher>::with_capacity_and_hasher(vec.len(), FxBuildHasher::default());
        let indices = vec.into_iter().map(|value| values.insert_full(value).0).collect();

        Self {
            values,
            indices,
        }
    }

    /// `IndexedVecXs`からVec<&VecX<T, N>>を生成します。
    pub fn to_ref_vec(&self) -> Vec<&VecX<T, N>> {
        self.indices.iter().map(|i| self.values.get_index(*i).unwrap()).collect::<Vec<_>>()
    }
}

impl<T: PartialEq + Eq + Hash + Copy, const N: usize> IndexedVecXs<T, N> {
    /// `IndexedVecXs`を`Vec<VecX<T, N>>`に変換します。
    pub fn to_vec(self) -> Vec<VecX<T, N>> {
        self.indices.into_iter().map(|i| *self.values.get_index(i).unwrap()).collect::<Vec<_>>()
    }
}


impl<T: PartialEq + Eq + Hash, const N: usize> Index<usize> for IndexedVecXs<T, N> {
    type Output = VecX<T, N>;

    fn index(&self, index: usize) -> &Self::Output {
        let i = self.indices.get(index).unwrap();
        self.values.get_index(*i).unwrap()
    }
}

/// `IndexedVecXs`のイテレータです。
/// `IndexedVecXs`の要素を順番に返します。
pub struct IndexedVecXIter<'a, T: PartialEq + Eq + Hash, const N: usize> {
    collection: &'a IndexedVecXs<T, N>,
    current_index: usize,
}

impl<'a, T: PartialEq + Eq + Hash, const N: usize> Iterator for IndexedVecXIter<'a, T, N> {
    type Item = &'a VecX<T, N>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.collection.indices.len() {
            self.current_index += 1;
            Some(&self.collection[self.current_index])
        } else {
            None
        }
    }
}