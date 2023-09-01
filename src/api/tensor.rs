// High level user API, exposed as it acts as the toolkit itself

use std::{
    fmt::{self, Debug, Display},
    ops,
};

use crate::{
    core::{Bundle, Operation},
    types::{Component, Packet},
};

/// Denoting shape a.k.a. dimensions of a `Tensor`'s `TensorMeta`.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TensorOrder {
    src: Vec<usize>,
}

impl TensorOrder {
    #[inline]
    fn fetch(&self) -> &Vec<usize> {
        &self.src
    }

    #[inline]
    pub fn count(&self) -> usize {
        self.src.iter().sum()
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.src.iter().product()
    }

    #[inline]
    pub fn square(&self) -> bool {
        self.src.iter().all(|dim| dim == &self.src[0])
    }
}

impl Display for TensorOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tensor of shape: {:?}.", self.src)
    }
}

/// Source container of a `Tensor`'s data, either owned or referenced
pub(crate) struct TensorMeta<'s, T, const N: usize>
where
    T: Component,
    Bundle<T>: Packet,
{
    src: Option<&'s [T]>,
    per: Option<Vec<T>>,
}

impl<'s, T, const N: usize> TensorMeta<'s, T, N>
where
    T: Component,
    Bundle<T>: Packet,
{
    #[inline]
    pub fn from_reference(_src: &'s [T]) -> Self {
        Self {
            src: Some(_src),
            per: None,
        }
    }

    #[inline]
    pub fn from_persist(_src: [T; N]) -> Self {
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
pub struct Tensor<'s, T, const N: usize>
where
    T: Component,
    Bundle<T>: Packet,
{
    pub order: TensorOrder,

    #[doc(hidden)]
    bundle: Bundle<T>,

    #[doc(hidden)]
    meta: TensorMeta<'s, T, N>,
}

impl<'s, T, const N: usize> Tensor<'s, T, N>
where
    T: Component,
    Bundle<T>: Packet,
{
    #[inline]
    fn _fetch(&self) -> &Bundle<T> {
        &self.bundle
    }

    pub fn from_array(_src: [T; N], order: TensorOrder) -> Self {
        let bundle = Bundle::bind_init(order.fetch()).unwrap();
        let meta = TensorMeta::from_persist(_src);

        Self {
            order,
            bundle,
            meta,
        }
    }

    pub fn from_slice(_src: &'s [T], order: TensorOrder) -> Self {
        let bundle = Bundle::bind_init(order.fetch()).unwrap();
        let meta = TensorMeta::from_reference(_src);

        Self {
            order,
            bundle,
            meta,
        }
    }

    pub fn cast<C: Component>(&mut self) {}
    pub fn determinant(&self) {}
    pub fn inverse(&self) {}

    /// Denotes a `TensorMeta` of valid slots, initialized.
    pub fn ready(&self) -> bool {
        let (src, per) = self.meta.slots();
        src || per
    }
}

macro_rules! impl_ops {
    ( $ ( $trait:ident $fn:ident, )* ) => {
        $ (
            impl<'s, T, const N: usize> ops::$trait for Tensor<'s, T, N> where
                T: Component,
            {
                type Output = Tensor<'s, T, N>;

                fn $fn(self, other: Tensor<'s, T, N>) -> Self::Output {
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

#[macro_export]
macro_rules! tsr {

    ( $root:literal $ (, $next:literal )* $(,)? ) => {
        {
            let _src = [$root $ (, $next )*];
            let order = TensorOrder::Vector(vec![_src.len() as u64]);
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

            let order = TensorOrder::Matrix(vec![x, y]);
            Tensor::from_array(_src, order)
        }
    };

}
