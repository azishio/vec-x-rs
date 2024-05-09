use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use num::Num;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VecX<T, const N: usize> {
    pub data: [T; N],
}

impl<T, const N: usize> VecX<T, N> {
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
