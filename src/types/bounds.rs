use bytemuck::NoUninit;

use crate::core::Bundle;

pub(crate) mod _sealed {
    pub trait Sealed {}
}

/// Valid types for shaders to operate on.
pub trait Component: _sealed::Sealed + NoUninit {}

/// Valid types to pack into a `Bundle`.
pub trait Packet: _sealed::Sealed {}

macro_rules! impl_component {
    ($($ty:ident)*) => {$(
        impl Component for $ty {}
        impl _sealed::Sealed for $ty {}
    )*}
}

macro_rules! impl_packet {
    ($($ty:ident)*) => {$(
        impl Packet for Bundle<$ty> {}
        impl _sealed::Sealed for Bundle<$ty> {}
    )*}
}

impl_component! {
    u16 u32 u64
    i16 i32 i64
    f32 f64
}

impl_packet! {
    f32 f64
}
