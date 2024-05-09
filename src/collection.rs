use std::hash::Hash;
use std::ops::Index;

use fxhash::FxBuildHasher;
use indexmap::IndexSet;

use crate::VecX;

pub trait VecXCollection {}

impl<T, const N: usize> VecXCollection for Vec<VecX<T, N>> {}

pub struct IndexedVecXs<T, const N: usize> {
    pub values: IndexSet<VecX<T, N>, FxBuildHasher>,
    pub indices: Vec<usize>,
}

impl<T, const N: usize> VecXCollection for IndexedVecXs<T, N> {}

impl<T: PartialEq + Eq + Hash, const N: usize> IndexedVecXs<T, N> {
    pub fn new(
        values: IndexSet<VecX<T, N>, FxBuildHasher>,
        indices: Vec<usize>) -> Self {
        Self {
            values,
            indices,
        }
    }

    pub fn empty() -> Self {
        Self {
            values: IndexSet::<VecX<T, N>, FxBuildHasher>::default(),
            indices: Vec::new(),
        }
    }
    pub fn iter(&self) -> IndexedVecXIter<T, N> {
        IndexedVecXIter {
            collection: self,
            current_index: 0,
        }
    }

    pub fn from_vec(vec: Vec<VecX<T, N>>) -> Self {
        let mut values = IndexSet::<VecX<T, N>, FxBuildHasher>::with_capacity_and_hasher(vec.len(), FxBuildHasher::default());
        let indices = vec.into_iter().map(|value| values.insert_full(value).0).collect();

        Self {
            values,
            indices,
        }
    }
}

impl<T, const N: usize> Index<usize> for IndexedVecXs<T, N> {
    type Output = VecX<T, N>;

    fn index(&self, index: usize) -> &Self::Output {
        let i = self.indices.get(index).unwrap();
        self.values.get_index(*i).unwrap()
    }
}

pub struct IndexedVecXIter<'a, T, const N: usize> {
    collection: &'a IndexedVecXs<T, N>,
    current_index: usize,
}

impl<'a, T, const N: usize> Iterator for IndexedVecXIter<'a, T, N> {
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
