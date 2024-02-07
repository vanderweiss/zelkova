#![allow(non_camel_case_types)]

use bytemuck::NoUninit;

#[repr(C)]
#[derive(Clone, Copy, Debug, NoUninit)]
pub struct f16(u16);
