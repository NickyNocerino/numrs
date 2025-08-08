use std::ops;

use crate::*;

#[derive(Clone, Debug)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    values:Vec<f64>
}


impl From<ArrayND<f64>> for Matrix {
    fn from(item: ArrayND<f64>) -> Self {
        let shape = item.shape();
        if !shape.len() == 2 {
            panic!("{}", format!("Illegal conversion, trying to cast ArrayND of shape {:?} to matrix", item.shape()));
        }
        Self{
            rows: shape[0],
            cols: shape[1],
            values: item.get_flat_data(),
        }
    }
}

impl Matrix {
    pub fn shape(&self) -> Vec<usize> {
        vec![self.rows, self.cols]
    }

    pub fn get_flat_data(&self) -> Vec<f64> {
        self.values.clone()
    }

    pub fn mat_mul(&self, other: &Matrix) -> Result<Self, MatrixError> {
        if !(self.cols == other.rows) {
            return Err(MatrixError::MissmatchedDimensions(format!("Illegal Operation, cannot multiply matrix {:?} by matrix {:?}", self, other)));
        }
        let mut out_values = vec![0.0;self.rows*other.cols];

        Ok(Self {
            rows: self.rows,
            cols: other.cols,
            values: out_values
        })
    }

}