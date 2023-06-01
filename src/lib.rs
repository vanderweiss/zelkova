#![feature(map_try_insert)]
#![feature(once_cell)]
#![feature(ptr_from_ref)]

pub(crate) mod api;
pub(crate) mod codegen;

pub mod models;

pub use self::models::{Tensor, TensorOrder};
