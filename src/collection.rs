use std::hash::Hash;
use std::ops::Index;

use fxhash::FxBuildHasher;
use indexmap::IndexSet;

use crate::vec_x::VecX;

trait VexXCollection {}

impl<T, const N: usize> VexXCollection for VecX<T, N> {}

pub struct IndexedVecXCollection<T, const N: usize> {
    pub values: IndexSet<VecX<T, N>, FxBuildHasher>,
    pub indices: Vec<usize>,
}

impl<T, const N: usize> VexXCollection for IndexedVecXCollection<T, N> {}

impl<T: PartialEq + Eq + Hash, const N: usize> IndexedVecXCollection<T, N> {
    pub fn from_vec(vec: Vec<VecX<T, N>>) -> Self {
        let mut values = IndexSet::<VecX<T, N>, FxBuildHasher>::with_capacity_and_hasher(vec.len(), FxBuildHasher::default());
        let indices = vec.into_iter().map(|value| values.insert_full(value).0).collect();

        Self {
            values,
            indices,
        }
    }
}

impl<T, const N: usize> Index<usize> for IndexedVecXCollection<T, N> {
    type Output = VecX<T, N>;

    fn index(&self, index: usize) -> &Self::Output {
        let i = self.indices.get(index).unwrap();
        self.values.get_index(*i).unwrap()
    }
}


