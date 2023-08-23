use std::marker::PhantomData;

use crate::internals::Component;

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

pub(crate) struct Operation {
    state: State,
    ty: Shader,
}

impl Operation {
    pub fn resolved(&self) -> bool {
        match self.state {
            State::Pending => false,
            State::Done => true,
        }
    }
}
