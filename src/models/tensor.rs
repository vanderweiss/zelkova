// High level user API, exposed as it acts as the toolkit itself

use std::{fmt, ops};

use crate::{api::Bundle, internals::Component};

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

pub(crate) struct TensorMeta<'s, C: Component, const N: usize> {
    src: Option<&'s [C]>,
    per: Option<Vec<C>>,
}

impl<'s, C: Component, const N: usize> TensorMeta<'s, C, N> {
    #[inline]
    pub fn from_reference(_src: &'s [C]) -> Self {
        Self {
            src: Some(_src),
            per: None,
        }
    }

    #[inline]
    pub fn from_persist(_src: [C; N]) -> Self {
        Self {
            src: None,
            per: Some(Vec::from(_src)),
        }
    }

    #[inline]
    pub fn slots(&self) -> (bool, bool) {
        (self.src.is_some(), self.per.is_some())
    }
}

/// Most basic element in the toolkit, composing every model.
pub struct Tensor<'s, C: Component, const N: usize> {
    pub order: TensorOrder,

    #[doc(hidden)]
    bundle: Bundle,

    #[doc(hidden)]
    meta: TensorMeta<'s, C, N>,
}

impl<'s, C: Component, const N: usize> Tensor<'s, C, N> {
    fn _fetch(&self) -> &Bundle {
        &self.bundle
    }

    pub fn from_array(_src: [C; N], order: TensorOrder) -> Self {
        let bundle = Bundle::bind_st::<C>(N).unwrap();
        let meta = TensorMeta::from_persist(_src);

        Self {
            order,
            bundle,
            meta,
        }
    }

    pub fn from_slice(_src: &'s [C], order: TensorOrder) -> Self {
        let bundle = Bundle::bind_st::<C>(_src.len()).unwrap();
        let meta = TensorMeta::from_reference(_src);

        Self {
            order,
            bundle,
            meta,
        }
    }

    pub fn cast<T: Component>(&mut self) {}

    /// hyperdeterminant for 3D+
    pub fn determinant(&self) {}

    pub fn inverse(&self) {}
}

#[doc(hidden)]
macro_rules! impl_ops {
    ( $ ( $trait:ident $fn:ident, )* ) => {
        $ (
            impl<'s, C: Component, const N: usize> ops::$trait for Tensor<'s, C, N> {
                type Output = Tensor<'s, C, N>;

                fn $fn(self, other: Tensor<'s, C, N>) -> Self::Output {
                    let (lb, rb) = (self._fetch(), other._fetch());
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
