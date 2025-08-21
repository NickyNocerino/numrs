use std::ops;

use crate::*;

pub type Tensor = ArrayND<f64>;

impl From<Matrix> for Tensor {
    fn from(item:Matrix) -> Self {
        Self::new(
            item.shape(),
            item.get_flat_data(),
        ).expect("Matrix should be legal")
    }
}

impl ops::Add for Tensor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.elementwise_add(&other).expect("MISSMATCHING TENSORS")
    }
}

impl ops::Sub for Tensor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.elementwise_sub(&other).expect("MISSMATCHING TENSORS")
    }
}

impl Tensor {
    pub fn elementwise_add(&self, other: &Self) -> Result<Self, TensorError> {
         if self.shape() != other.shape() {
            return Err(TensorError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let flat_data_1 = self.get_flat_data();
        let flat_data_2 = other.get_flat_data();
        let mut out_values = vec![];
        for i in 0..flat_data_1.len() {
            out_values.push(flat_data_1[i] + flat_data_2[i]);
        }
        match Self::new(self.shape(), out_values) {
            Ok(out) => {return Ok(out);},
            Err(e) => {
                return Err(TensorError::DependencyError(format!("{:?}", e)));
            }
        }
        
    }

    pub fn elementwise_sub(&self, other: &Self) -> Result<Self, TensorError> {
         if self.shape() != other.shape() {
            return Err(TensorError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let flat_data_1 = self.get_flat_data();
        let flat_data_2 = other.get_flat_data();
        let mut out_values = vec![];
        for i in 0..flat_data_1.len() {
            out_values.push(flat_data_1[i] - flat_data_2[i]);
        }
        match Self::new(self.shape(), out_values) {
            Ok(out) => {return Ok(out);},
            Err(e) => {
                return Err(TensorError::DependencyError(format!("{:?}", e)));
            }
        }
        
    }

    pub fn elementwise_mult(&self, other: &Self) -> Result<Self, TensorError> {
         if self.shape() != other.shape() {
            return Err(TensorError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let flat_data_1 = self.get_flat_data();
        let flat_data_2 = other.get_flat_data();
        let mut out_values = vec![];
        for i in 0..flat_data_1.len() {
            out_values.push(flat_data_1[i] * flat_data_2[i]);
        }
        match Self::new(self.shape(), out_values) {
            Ok(out) => {return Ok(out);},
            Err(e) => {
                return Err(TensorError::DependencyError(format!("{:?}", e)));
            }
        }
        
    }
    pub fn elementwise_div(&self, other: &Self) -> Result<Self, TensorError> {
         if self.shape() != other.shape() {
            return Err(TensorError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let flat_data_1 = self.get_flat_data();
        let flat_data_2 = other.get_flat_data();
        let mut out_values = vec![];
        for i in 0..flat_data_1.len() {
            if flat_data_2[i] == 0.0 {
                return Err(TensorError::DivideByZero)
            }
            out_values.push(flat_data_1[i] / flat_data_2[i]);
        }
        match Self::new(self.shape(), out_values) {
            Ok(out) => {return Ok(out);},
            Err(e) => {
                return Err(TensorError::DependencyError(format!("{:?}", e)));
            }
        }
    }
}