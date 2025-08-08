use std::ops;

use crate::*;

#[derive(Clone)]
pub struct Matrix {
    shape: (usize, usize),
    values:Vec<f64>
}


impl From<ArrayND<f64>> for Matrix {
    fn from(item: ArrayND<f64>) -> Self {
        let shape = item.shape();
        if !shape.len() == 2 {
            panic!("{}", format!("Illegal conversion, trying to cast ArrayND of shape {:?} to matrix", item.shape()));
        }
        Self{
            shape: (shape[0], shape[1]),
            values: item.get_flat_data(),
        }
    }
}