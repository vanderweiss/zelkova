#![feature(map_try_insert)]
#![feature(lazy_cell)]
#![feature(ptr_from_ref)]

pub(crate) mod api;
pub(crate) mod codegen;
pub(crate) mod internals;

pub mod models;

pub use self::models::{Tensor, TensorOrder};
