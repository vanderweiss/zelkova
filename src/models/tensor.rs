// High level user API, exposed as it acts as the toolkit itself

use std::{
    fmt, ops,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{
    api::Bundle,
    codegen::{Component, _Tty},
};

static TRACKER: AtomicU32 = AtomicU32::new(0);

#[derive(PartialEq, Eq, Debug)]
pub enum TensorOrder {
    Scalar,
    Vector(u64),
    Matrix(u64, u64),
    Cube(u64, u64, u64),
}

impl TensorOrder {
    #[inline]
    pub fn size(&self) -> u64 {
        match self {
            Self::Scalar => 1,
            Self::Vector(x) => x * 1,
            Self::Matrix(x, y) => x * y,
            Self::Cube(x, y, z) => x * y * z,
        }
    }

    #[inline]
    pub fn square(&self) -> bool {
        match self {
            Self::Scalar => false,
            Self::Vector(_) => false,
            Self::Matrix(x, y) => x == y,
            Self::Cube(x, y, z) => (x == y) && (y == z),
        }
    }
}

impl fmt::Display for TensorOrder {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Scalar => {
                write!(f, "Tensor has implied shape.")
            }
            Self::Vector(x) => {
                write!(f, "Tensor has shape ({})", x)
            }
            Self::Matrix(x, y) => {
                write!(f, "Tensor has shape ({}, {})", x, y)
            }
            Self::Cube(x, y, z) => {
                write!(f, "Tensor has shape ({}, {}, {})", x, y, z)
            }
        }
    }
}

pub struct Tensor<C: Component, const N: usize> {
    pub _tensor: [C; N],
    pub _index: _Tty,

    pub order: TensorOrder,
}

impl<C: Component, const N: usize> Tensor<C, N> {
    #[inline]
    fn _prepare(&self) -> &Bundle {
        Bundle::bind(&self._tensor, self._index).unwrap()
    }

    #[inline]
    pub fn raw(_tensor: [C; N], order: TensorOrder) -> Self {
        let _index: _Tty = TRACKER.fetch_add(1, Ordering::SeqCst);

        Self {
            _tensor,
            _index,
            order,
        }
    }

    pub fn cast<T: Component>(&mut self) {}

    // hyperdeterminant for 3D+
    pub fn determinant(&self) {}

    pub fn inverse(&self) {}
}

macro_rules! impl_ops {
    ( $ ( $trait:ident $fn:ident )*, ) => {
        $ (
            impl<C: Component, const N: usize> ops::$trait for Tensor<C, N> {
                type Output = Tensor<C, N>;

                fn $fn(self, rhs: Tensor<C, N>) -> Self::Output {
                }
            }
        )*
    };
}

impl_ops! {
    Add add,
}

// vec! but tensor, limited to second rank
#[macro_export]
macro_rules! tsr {

    ( $root:literal $ (, $next:literal )* $(,)? ) => {
        {
            let _tensor = [
                $root $ (, $next )*
            ];

            let order = TensorOrder::Vector(_tensor.len() as u64);

            Tensor::raw(_tensor, raw)
        }

    };

    ( $ ( [ $root:literal $ (, $next:literal)* ] $(,)? )*  ) => {
        {
            let (mut x, mut y) = (1, 0);
            let mut depth = true;
            let _tensor = [
                $ (
                    {
                        y += 1;
                        if y > 1 { depth = !depth; }
                        $root
                    },
                    $ (
                        {
                            depth.then(|| x += 1);
                            $next
                        },
                    )*
                )*
            ];

            let order = TensorOrder::Matrix(x as u64, y as u64);

            Tensor::raw(_tensor, order)
        }
    };

}
