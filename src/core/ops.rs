use std::marker::PhantomData;

use super::Bundle;

use crate::types::{Component, Packet};

#[derive(Clone, Copy, Default)]
enum State {
    #[default]
    Pending,
    Done,
}

#[derive(Clone, Copy)]
enum ElementType {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
    Rot,
}

#[derive(Clone, Copy)]
enum DimensionalType {
    Sum,
    Determinant,
    Inverse,
    Transpose,
}

#[derive(Clone, Copy)]
enum BoundCPU {}

#[derive(Clone, Copy)]
enum BoundGPU {}

#[derive(Clone, Copy)]
pub(crate) enum Shader {
    Element(ElementType),
    Dimensional(DimensionalType),
}

#[derive(Clone, Copy)]
pub(crate) enum Workgroup {
    Single(u32),
    Duplet(u32, u32),
    Triplet(u32, u32, u32),
}

impl Workgroup {
    pub fn collapse(&self) -> u32 {
        match self {
            Workgroup::Single(x) => *x,
            Workgroup::Duplet(x, y) => x * y,
            Workgroup::Triplet(x, y, z) => x * y * z,
        }
    }
}

pub(crate) struct Operation<T>
where
    T: Component,
    Bundle<T>: Packet,
{
    pub state: State,
    pub workgroup: Workgroup,
    pub ty: Shader,

    target: PhantomData<T>,
}

impl<T> Operation<T>
where
    T: Component,
    Bundle<T>: Packet,
{
    pub fn new<L, R, Lhs, Rhs>(lhs: &Bundle<L>, rhs: &Bundle<R>, ty: Shader) -> Self
    where
        L: Component,
        R: Component,
    {
        let dims: Vec<u32> = { if lhs.props.dims == rhs.props.dims {} };
    }

    pub fn resolved(&self) -> bool {
        match self.state {
            State::Pending => false,
            State::Done => true,
        }
    }
}
