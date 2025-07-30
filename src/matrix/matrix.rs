use std::ops;

use crate::*;

pub struct MatrixND {
    values: ArrayND<f64>
}

impl ops::Add<MatrixND> for MatrixND {
    type Output = Result<Self, MatrixNDError>;

    fn add(self, other: MatrixND) -> Result<Self, MatrixNDError> {

        if !(self.values.shape() == other.values.shape()){
            return Err(MatrixNDError::MissmatchedDimensions(
                format!{
                    "Illegal operation: matrix of shape {:?} + matrix of shape: {:?}",
                    self.values.shape(),
                    other.values.shape()
                }
            ));
        }

        return Ok(Self{
            values:ArrayND::<f64>::new(
                self.values.shape(), 
                self.values.get_flat_data().iter().zip(other.values.get_flat_data().iter()).map(|(&a, &b)| a + b).collect())
            }
        );
    }
}

impl ops::Sub<MatrixND> for MatrixND {
    type Output = Result<Self, MatrixNDError>;

    fn sub(self, other: MatrixND) -> Result<Self, MatrixNDError> {

        if !(self.values.shape() == other.values.shape()){
            return Err(MatrixNDError::MissmatchedDimensions(
                format!{
                    "Illegal operation: matrix of shape {:?} - matrix of shape: {:?}",
                    self.values.shape(),
                    other.values.shape()
                }
            ));
        }

        Ok(Self{
            values:ArrayND::<f64>::new(
                self.values.shape(), 
                self.values.get_flat_data().iter().zip(other.values.get_flat_data().iter()).map(|(&a, &b)| a - b).collect())
            }
        )
    }
}

impl MatrixND {
    pub fn zeros(shape: Vec<usize>) -> Self{
        Self{
            values:ArrayND::fill(shape, 0.0)
        }
    }

    pub fn ones(shape: Vec<usize>) -> Self{
        Self{
            values:ArrayND::fill(shape, 1.0)
        }
    }


}