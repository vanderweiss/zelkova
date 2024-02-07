use bytemuck::NoUninit;
use std::marker::PhantomData;

use super::extension::f16;

pub(crate) mod _sealed {
    pub trait Sealed {}
}

/// Valid types for models to operate on.
pub trait Component: _sealed::Sealed + NoUninit {}

/// Valid types to pack into a `Bundle`. Wrapper around abstract types e.g. f16 and implicit
/// casts depending on the specifications.
pub struct Packet<T: Component>(PhantomData<T>);

pub trait SupportedPacket: _sealed::Sealed {}

macro_rules! impl_sealed {
    ($($ty:ident)*) => {$(
        impl _sealed::Sealed for $ty {}
    )*}
}

macro_rules! impl_component {
    ($($ty:ident)*) => {$(
        impl Component for $ty {}
    )*}
}

macro_rules! impl_packet {
    ($($ty:ident)*) => {$(
        impl SupportedPacket for Packet<$ty> {}
        impl _sealed::Sealed for Packet<$ty> {}
    )*}
}

impl_sealed! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

impl_component! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

impl_packet! {
    u32
    i32
    f32
}
