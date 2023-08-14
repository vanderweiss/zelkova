//! A vendor-neutral GPU library that aims to provide a simple and straightforward alternative to
//! modern frameworks for machine learning and AI-related workload.

//#![feature(const_trait_impl)]
//#![feature(lazy_cell)]
//#![feature(map_try_insert)]
//#![feature(ptr_from_ref)]
#![allow(non_upper_case_globals)]

pub(crate) mod api;
pub(crate) mod codegen;
pub(crate) mod internals;

pub mod models;

pub use self::models::{Tensor, TensorOrder};

pub fn init() -> api::Instance {
    api::Instance::init().unwrap()
}
