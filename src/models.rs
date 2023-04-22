// High level user API, exposed as it acts as the toolkit itself

use super::interface::*;

#[derive(Debug)]
pub enum TensorRank {
    Scalar,
    Vector(u64),
    Matrix(u64, u64),
    Cube(u64, u64, u64),
}

impl TensorRank {}

#[derive(Debug)]
pub struct Tensor<T: Element, const N: usize> {
    _tensor: [T; N],
    _rank: TensorRank,
}

impl<T: Element, const N: usize> Tensor<T, N> {
    pub fn cast(&self) {}

    pub fn _resize(&self) {}
}

// vec! yoink ez 
#[macro_use]
macro_rules! tensor {
    () => {

    }
}

