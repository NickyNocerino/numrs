use crate::*;

#[derive(Clone, Debug)]
pub struct Tensor {
    values: ArrayND<f64>,
}

impl From<Matrix> for Tensor {
    fn from(item:Matrix) -> Self {
        Self {
            values: ArrayND::<f64>::new(item.shape(), item.get_flat_data()).expect("ILLEGAL MATRIX")
        }
    }
}