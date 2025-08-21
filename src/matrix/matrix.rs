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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.rows == other.rows && self.cols == other.cols{
            let other_data = other.get_flat_data();
            for (v1, v2) in self.get_flat_data().iter().zip(&other_data) {
                if v1 != v2 {
                    return false;
                }
            }
            return true;

        }
        false
    }
}


impl ops::Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        self.elementwise_add(&other).expect("MISMATCHING MATRIXES")
    }
}

impl ops::Sub for Matrix {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        self.elementwise_sub(&other).expect("MISMATCHING MATRIXES")
    }
}

impl ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.elementwise_mult(&other).expect("MISMATCHING MATRIXES")
    }
}

impl ops::Div for Matrix {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        self.elementwise_div(&other).expect("MISMATCHING MATRIXES")
    }
}

impl ops::BitXor for Matrix {
    type Output = Self;

    fn bitxor(self, other: Self) -> Self {
        self.matrix_mult(&other).expect("MISMATCHING MATRIXES")
    }
}

impl Matrix {
    
    pub fn zeros(rows: usize, cols: usize) -> Self {
        let mut data = Vec::<f64>::new();
        for _i in 0..(rows*cols) {
            data.push(0.0);
        }
        Self{
            rows:rows,
            cols:cols,
            values:data,
        }
        
    }


    pub fn ones(rows: usize, cols: usize) -> Self {
        let mut data = Vec::<f64>::new();
        for _i in 0..(rows*cols) {
            data.push(1.0);
        }
        Self{
            rows:rows,
            cols:cols,
            values:data,
        }
        
    }
    
    pub fn identity(rows: usize, cols: usize) -> Self {
        let mut data = Vec::<f64>::new();
        for i in 0..(rows) {
            for j in 0..cols {
                if i == j {
                    data.push(1.0);
                }
                else {
                    data.push(0.0);
                }
            }
        }
        
        Self{
            rows:rows,
            cols:cols,
            values:data,
        }
        
    
    }

    pub fn shape(&self) -> Vec<usize> {
        vec![self.rows, self.cols]
    }

    pub fn get_flat_data(&self) -> Vec<f64> {
        self.values.clone()
    }
    
    pub fn elementwise_add(&self, other: &Self) -> Result<Self, MatrixError> {
         if self.shape() != other.shape() {
            return Err(MatrixError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let mut out_values = vec![];
        for i in 0..self.values.len() {
            out_values.push(self.values[i] + other.values[i]);
        }
        Ok(Self {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            values:out_values,
        })
    }

    pub fn elementwise_sub(&self, other: &Self) -> Result<Self, MatrixError> {
         if self.shape() != other.shape() {
            return Err(MatrixError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let mut out_values = vec![];
        for i in 0..self.values.len() {
            out_values.push(self.values[i] - other.values[i]);
        }
        Ok(Self {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            values:out_values,
        })
    }

    pub fn elementwise_mult(&self, other: &Self) -> Result<Self, MatrixError> {
         if self.shape() != other.shape() {
            return Err(MatrixError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let mut out_values = vec![];
        for i in 0..self.values.len() {
            out_values.push(self.values[i] * other.values[i]);
        }
        Ok(Self {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            values:out_values,
        })
    }

    pub fn elementwise_div(&self, other: &Self) -> Result<Self, MatrixError> {
         if self.shape() != other.shape() {
            return Err(MatrixError::MissmatchedDimensions(format!("Illegal Operation, cannot add matrix of shape {:?} to matrix of shape {:?}", self.shape(), other.shape())));
        }
        let mut out_values = vec![];
        for i in 0..self.values.len() {
            if  other.values[i] == 0.0 {
                return Err(MatrixError::DivideByZero);
            }
            out_values.push(self.values[i] / other.values[i]);
        }
        Ok(Self {
            rows: self.rows.clone(),
            cols: self.cols.clone(),
            values:out_values,
        })
    }


    pub fn matrix_mult(&self, other: &Self) -> Result<Self, MatrixError> {
        if self.cols != other.rows {
            return Err(MatrixError::MissmatchedDimensions(format!("Illegal Operation, cannot multiply matrix of shape {:?} by matrix of shape {:?}", self.shape(), other.shape())));
        }
        let mut out_values = vec![0.0;self.rows * other.cols];

        for i in 0..self.rows {
            // Iterate through rows of A (and thus rows of C)
            for j in 0..other.cols {
                // Iterate through columns of B (and thus columns of C)
                for k in 0..self.cols {
                    // Iterate through columns of A (and rows of B)
                    // Access elements: data_matrix[row_index * num_cols + col_index]
                    let a_val = self.values[i * self.cols + k];
                    let b_val = other.values[k * other.cols + j];
                    out_values[i * other.cols + j] += a_val * b_val;
                }
            }
        }
        Ok(Self {
            rows: self.rows,
            cols: other.cols,
            values: out_values,
        })
    }

    pub fn transpose(&mut self) {
        let mut transposed_data = vec![0.0; self.rows * self.cols]; 

        for r in 0..self.rows {
            for c in 0..self.cols {
                let original_index = r * self.cols + c; 
                let transposed_index = c * self.rows + r; 
                transposed_data[transposed_index] = self.values[original_index];
            }
        }
        *self = Self{
            rows: self.cols.clone(),
            cols: self.rows.clone(),
            values: transposed_data,
        }

    }

    pub fn transposed_clone(&self) -> Self {
        let mut transposed_data = vec![0.0; self.rows * self.cols]; 

        for r in 0..self.rows {
            for c in 0..self.cols {
                let original_index = r * self.cols + c; 
                let transposed_index = c * self.rows + r; 
                transposed_data[transposed_index] = self.values[original_index];
            }
        }
        Self {
            rows: self.cols.clone(),
            cols: self.rows.clone(),
            values: transposed_data,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_fill() {
        let _zeros: Matrix = Matrix::zeros(4,4);
        let _ones: Matrix = Matrix::ones(4,4);
        let _identity: Matrix = Matrix::identity(4,4);
    }

    #[test]
    fn test_ops() {
        let zeros: Matrix = Matrix::zeros(4,4);
        let ones: Matrix = Matrix::ones(4,4);
        
        assert_eq!(ones.clone(), ones.clone() + zeros.clone());

        assert_eq!(ones.clone(), ones.clone() - zeros.clone());

        assert_eq!(zeros.clone(), ones.clone() * zeros.clone());

        assert_eq!(ones.clone(), ones.clone() * ones.clone());

        assert_eq!(ones.clone(), ones.clone() / ones.clone());

        assert_eq!(zeros.clone(), ones.clone() ^ zeros.clone());
    }

    #[test]
    fn test_transpose() {
        let matrix: Matrix = Matrix::identity(5,5);
        assert_eq!(matrix, matrix.transposed_clone())
    }

}