//! A vendor-neutral GPU library that aims to provide a simple and straightforward alternative to
//! modern frameworks for machine learning and AI-related workload.

#![allow(non_upper_case_globals)]

pub mod api;
pub(crate) mod core;
pub(crate) mod internals;
pub(crate) mod shaders;
pub(crate) mod types;

pub use self::api::{Tensor, TensorOrder};

pub fn init() -> core::Instance {
    core::Instance::init().unwrap()
}
