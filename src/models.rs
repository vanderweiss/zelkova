// High level user API, exposed as it acts as the toolkit itself

use std::{
    fmt, ops,
    sync::atomic::{AtomicU32, Ordering},
};

use super::shader::Component;

static TRACKER: AtomicU32 = AtomicU32::new(0);

#[derive(PartialEq, Eq, Debug)]
pub enum TensorRank {
    Scalar,
    Vector(u64),
    Matrix(u64, u64),
    Cube(u64, u64, u64),
}

impl TensorRank {
    #[inline]
    pub fn size(&self) -> u64 {
        match self {
            TensorRank::Scalar => 1,
            TensorRank::Vector(x) => x * 1,
            TensorRank::Matrix(x, y) => x * y,
            TensorRank::Cube(x, y, z) => x * y * z,
        }
    }
}

impl fmt::Display for TensorRank {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TensorRank::Scalar => {
                write!(f, "Tensor has implied shape.")
            }
            TensorRank::Vector(x) => {
                write!(f, "Tensor has shape ({})", x)
            }
            TensorRank::Matrix(x, y) => {
                write!(f, "Tensor has shape ({}, {})", x, y)
            }
            TensorRank::Cube(x, y, z) => {
                write!(f, "Tensor has shape ({}, {}, {})", x, y, z)
            }
        }
    }
}

pub struct Tensor<C: Component, const N: usize> {
    pub _tensor: [C; N],
    pub _index: u32,

    pub rank: TensorRank,
}

impl<C: Component, const N: usize> Tensor<C, N> {
    pub fn _prepare() {}

    #[inline]
    pub fn raw(_tensor: [C; N], rank: TensorRank) -> Self {
        let _index: u32 = TRACKER.fetch_add(1, Ordering::SeqCst);

        Self {
            _tensor,
            _index,
            rank,
        }
    }

    pub fn cast(&self) {}
}

/* WIP
macro_rules! impl_ops {
    ( $ ( $trait:ident $fn:ident )*, ) => {
        $ (
            impl<C: Component, const N: usize> ops::$trait for Tensor<C, N> {
                type Output = Tensor<C, N>;

                fn $fn(&self, rhs: &Tensor<C, N>) -> Output {
                    let (lb, rb) = (self._prepare(), rhs._prepare());
                }
            }
        )*
    };
}


impl_ops! {
    Add add,
}
*/

// vec! but tensor, limited to second rank
#[macro_export]
macro_rules! tsr {

    ( $root:literal $ (, $next:literal )* $(,)? ) => {
        {
            let _tensor = [
                $root $ (, $next )*
            ];

            let rank = TensorRank::Vector(_tensor.len() as u64);

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

            let rank = TensorRank::Matrix(x as u64, y as u64);

            Tensor::raw(_tensor, rank)
        }
    };

}
