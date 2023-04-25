// High level user API, exposed as it acts as the toolkit itself

use {
    std::{
        ops::{self},
    }
};

use super::interface::*;

pub enum TensorRank {
    Scalar,
    Vector(u64),
    Matrix(u64, u64),
    Cube(u64, u64, u64),
}

impl TensorRank {}

pub struct Tensor<T: Element, const N: usize> {
    pub _tensor: [T; N],
    pub _rank: TensorRank,
}

impl<T: Element, const N: usize> Tensor<T, N> {

    pub fn from_array(array: [T; N], rank: TensorRank) -> Self {
        Self {
            _tensor: array,
            _rank: rank,
        }
    }
    

    pub fn cast(&self) {}

    pub fn _resize(&self) {}
}

// vec! yoink ez 
#[macro_export]
macro_rules! tensor {
    
    () => {};

    ( $root:literal $ (, $next:literal )* $(,)? ) => {
        || { 
            let _tensor = [
                $root $ (
                    , $next
                )*
            ];
            
            let _rank = TensorRank::Vector(_tensor.len() as u64);
            
            Tensor {
                _tensor, 
                _rank,
            }
        }
            
    };

    ( $ ( [ $root:literal $ (, $next:literal)* ] $(,)? )*  ) => { 
        || {
            let (mut x, mut y) = (0, 0);
            let _tensor = [
                $ (
                    || {
                        x = x + 1;
                        $root
                    }, 
                    $ (
                        || {
                            y = y + 1;
                            $next
                        },
                    )*
                )*
            ];

            let _rank = TensorRank::Matrix(x as u64, y as u64);
            
            Tensor {
                _tensor,
                _rank,
            }
        }
    };

}
