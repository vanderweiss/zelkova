// High level user API, exposed as it acts as the toolkit itself

use {
    std::{
        fmt,
        ops,
    }
};

use super::interface::*;

#[derive(Debug)]
pub enum TensorRank {
    Scalar,
    Vector(u64),
    Matrix(u64, u64),
    Cube(u64, u64, u64),
}

impl TensorRank {
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TensorRank::Scalar => {
                write!(f, "Tensor has implied shape.")
            },
            TensorRank::Vector(x) => {
                write!(f, "Tensor has shape ({})", x)
            },
            TensorRank::Matrix(x, y) => {
                write!(f, "Tensor has shape ({}, {})", x, y)
            },
            TensorRank::Cube(x, y, z) => {
                write!(f, "Tensor has shape ({}, {}, {})", x, y, z)
            }
        }
    }
}

pub struct Tensor<T: Element, const N: usize> {
    pub _tensor: [T; N],
    pub rank: TensorRank,
}

impl<T: Element, const N: usize> Tensor<T, N> {

    pub fn from_array(_tensor: [T; N], rank: TensorRank) -> Self {
        Self {
            _tensor,
            rank,
        }
    }
    

    pub fn cast(&self) {}

    pub fn _resize(&self) {}
}

// vec! yoink ez 
#[macro_export]
macro_rules! tsr {
    
    () => {};

    ( $root:literal $ (, $next:literal )* $(,)? ) => {
        (|| { 
            let _tensor = [
                $root $ (
                    , $next
                )*
            ];
            
            let rank = TensorRank::Vector(_tensor.len() as u64);
            
            Tensor {
                _tensor, 
                rank,
            }
        })()
            
    };

    ( $ ( [ $root:literal $ (, $next:literal)* ] $(,)? )*  ) => { 
        (|| {
            let (mut x, mut y) = (0, 0);
            let _tensor = [
                $ (
                    (|| {
                        x = x + 1;
                        $root
                    })(), 
                    $ (

                        (|| {
                            y = y + 1;
                            $next
                        })(),
                    )*
                )*
            ];

            let rank = TensorRank::Matrix(x as u64, y as u64);
            
            Tensor {
                _tensor,
                rank,
            }
        })()
    };

}
