//! A vendor-neutral GPU library that aims to provide a simple and straightforward alternative to
//! modern frameworks for machine learning and AI-related workload.

#![feature(map_try_insert)]
#![feature(lazy_cell)]
#![feature(ptr_from_ref)]

pub(crate) mod api;
pub(crate) mod codegen;
pub(crate) mod internals;

pub mod models;

pub use self::models::{Tensor, TensorOrder};
