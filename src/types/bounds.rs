use bytemuck::NoUninit;
use std::marker::PhantomData;

use super::{bf16::bf16, f16::f16, f8::f8};

pub(crate) mod _sealed {
    pub trait Sealed {}
}

/// Valid types for models and shaders to operate on.
pub trait Component: _sealed::Sealed + NoUninit {}
/// Valid types for shaders to operate on.
pub trait Abstract: _sealed::Sealed + NoUninit {}

/// Valid types to pack into a `Bundle`; albeit the same for now, f8, f16 and bf16 would go in here.
pub struct Packet<T>(PhantomData<T>)
where
    T: Abstract + Component;

pub trait SupportedPacket: _sealed::Sealed {}

macro_rules! impl_component {
    ($($ty:ident)*) => {$(
        impl Component for $ty {}
        impl _sealed::Sealed for $ty {}
    )*}
}

macro_rules! impl_abstract {
    ($($ty:ident)*) => {$(
        impl Abstract for $ty {}
        impl _sealed::Sealed for $ty {}
    )*}
}

macro_rules! impl_packet {
    ($($ty:ident)*) => {$(
        impl SupportedPacket for Packet<$ty> {}
        impl _sealed::Sealed for Packet<$ty> {}
    )*}
}

impl_component! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

impl_abstract! {
    f8 f16 bf16
}

impl_packet! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
    f8 f16 bf16
}
