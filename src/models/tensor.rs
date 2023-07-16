// High level user API, exposed as it acts as the toolkit itself

use std::{
    fmt, ops,
    sync::atomic::{AtomicU32, Ordering},
};

use crate::{
    api::{Bundle, Node, NodeType, ShaderSource},
    internals::Component,
};

/// Global tracker for bindings generated in compute shaders.
static TRACKER: AtomicU32 = AtomicU32::new(0);

/// Most basic element in the toolkit, composing every model.
pub struct Tensor<C: Component, const N: usize> {
    pub order: TensorOrder,

    #[doc(hidden)]
    _bundle: Bundle,

    #[doc(hidden)]
    _src: [C; N],
}

impl<C: Component, const N: usize> Tensor<C, N> {
    fn _fetch(&self) -> &Bundle {
        &self._bundle
    }

    pub fn from_array(_src: [C; N], order: TensorOrder) -> Self {
        let binding = TRACKER.fetch_add(1, Ordering::SeqCst);
        let _bundle = Bundle::bind(&_src, binding).unwrap();

        Self {
            order,
            _bundle,
            _src,
        }
    }

    pub fn cast<T: Component>(&mut self) {}

    /// hyperdeterminant for 3D+
    pub fn determinant(&self) {}

    pub fn inverse(&self) {}
}

/// Denoting shape a.k.a. dimensions of a tensor.
#[derive(PartialEq, Eq, Debug)]
pub enum TensorOrder {
    /// Tensors collapsed of implied dimensionality.
    Scalar,
    /// Tensors of 1D.
    Vector(u64),
    /// Tensors of 2D.
    Matrix(u64, u64),
    /// Tensors of 3D.
    Cube(u64, u64, u64),
}

impl TensorOrder {
    /// Matches shape to its own product.
    #[inline]
    pub fn size(&self) -> u64 {
        match self {
            Self::Scalar => 1,
            Self::Vector(x) => x * 1,
            Self::Matrix(x, y) => x * y,
            Self::Cube(x, y, z) => x * y * z,
        }
    }

    /// Checks for a square tensor, originally corresponding to the 2D space,
    /// now expanded upon higher dimensionality as a property.
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

#[doc(hidden)]
macro_rules! impl_ops {
    ( $ ( $trait:ident $fn:ident, )* ) => {
        $ (
            impl<C: Component, const N: usize> ops::$trait for Tensor<C, N> {
                type Output = Tensor<C, N>;

                fn $fn(self, other: Tensor<C, N>) -> Self::Output {
                    let (lb, rb) = (self._fetch(), other._fetch());

                    let source = ShaderSource::Toolkit("$fn");
                    let ty = NodeType::Arithmetic;

                    let node = Node::<'_, Bundle>::create(source, ty)
                        .include(lb, None)
                        .include(rb, None);

                    other
                }
            }
        )*
    };
}

impl_ops! {
    Add add,
    Sub sub,
    Mul mul,
    Div div,
}

// vec! but tensor, limited to second rank
#[doc(hidden)]
#[macro_export]
macro_rules! tsr {

    ( $root:literal $ (, $next:literal )* $(,)? ) => {
        {
            let _src = [$root $ (, $next )*];
            let order = TensorOrder::Vector(_src.len() as u64);
            Tensor::from_array(_tensor, raw)
        }

    };

    ( $ ( [ $root:literal $ (, $next:literal)* ] $(,)? )*  ) => {
        {
            let (mut x, mut y) = (1, 0);
            let mut depth = true;
            let _src = [
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
            Tensor::from_array(_src, order)
        }
    };

}
