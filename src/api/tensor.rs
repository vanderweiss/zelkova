// High level user API, exposed as it acts as the toolkit itself

use std::{
    fmt::{self, Debug, Display},
    ops,
};

use crate::{
    core::Bundle,
    types::{Component, Packet, SupportedPacket},
};

/// Denoting shape a.k.a. dimensions of a `Tensor`'s `TensorMeta`.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct TensorOrder {
    src: Vec<u32>,
}

impl TensorOrder {
    pub fn new(src: Vec<u32>) -> Self {
        Self { src }
    }

    #[inline]
    pub fn count(&self) -> u32 {
        self.src.iter().sum()
    }

    #[inline]
    pub fn pull(&self) -> Vec<u32> {
        self.src.clone()
    }

    #[inline]
    pub fn size(&self) -> u32 {
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
{
    src: Option<&'s [T]>,
    per: Option<Vec<T>>,
}

impl<'s, T, const N: usize> TensorMeta<'s, T, N>
where
    T: Component,
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
pub struct Tensor<'s, T, U, const N: usize>
where
    T: Component,
    Packet<U>: SupportedPacket,
{
    pub order: TensorOrder,

    bundle: Bundle<U>,
    meta: TensorMeta<'s, T, N>,
}

impl<'s, T, U, const N: usize> Tensor<'s, T, U, N>
where
    T: Component,
    Packet<U>: SupportedPacket,
{
    pub fn from_array(_src: [T; N], order: TensorOrder) -> Self {
        let bundle = Bundle::bind_init(order.pull()).unwrap();
        let meta = TensorMeta::from_persist(_src);

        Self {
            order,
            bundle,
            meta,
        }
    }

    pub fn from_slice(_src: &'s [T], order: TensorOrder) -> Self {
        let bundle = Bundle::bind_init(order.pull()).unwrap();
        let meta = TensorMeta::from_reference(_src);

        Self {
            order,
            bundle,
            meta,
        }
    }

    /*
    pub fn cast<'c: 's, C, const M: usize>(&mut self) -> Tensor<'c, C, M>
    where
        C: Component,
        Bundle<C>: Packet,
    {
    }
    */

    pub fn determinant(&self) {}
    pub fn inverse(&self) {}

    /// Pull internal `Bundle` representation.
    pub(crate) fn fetch(&self) -> &Bundle<U> {
        &self.bundle
    }

    /// Denotes a `TensorMeta` of valid slots, initialized.
    fn ready(&self) -> bool {
        let (src, per) = self.meta.slots();
        src || per
    }
}

macro_rules! impl_ops {
    ( $ ( $trait:ident $fn:ident, )* ) => {
        $ (
            impl<'s, T, U, const M: usize> ops::$trait for Tensor<'s, T, U, M> where
                T: Component,
                Packet<U>: SupportedPacket,
            {
                type Output = Tensor<'s, T, U, M>;

                fn $fn<'t, V, W, const N: usize>(self, other: Tensor<'t, V, W, N>) -> Self::Output where
                    V: Component,
                    Packet<W>: SupportedPacket,
                {
                    let (lb, rb) = (self.fetch(), other.fetch());
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
            let order = TensorOrder::new(vec![_src.len() as u32]);
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

            let order = TensorOrder::new(vec![x, y]);
            Tensor::from_array(_src, order)
        }
    };

}
