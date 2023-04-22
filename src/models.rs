// High level user API, exposed as it acts as the toolkit itself

use {
    bytemuck::NoUninit,
};

use super::interface::*;

mod element {
   pub trait Sealed {} 
}

pub trait Element: element::Sealed + NoUninit {}

macro_rules! impl_element {
    ($($ident:ident)*) => {$(
        impl Element for $ident {}
        impl element::Sealed for $ident {}
    )*}
}

impl_element! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

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

#[macro_use]
macro_rules! tensor {
    () => {

    }
}


