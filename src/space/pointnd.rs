use crate::*;

pub struct PointND {
    coords: ArrayND<f64>,
}

impl PointND{
    pub fn origin(n: usize) -> Self{
        Self{
            coords: ArrayND::fill(vec![n], 0.0)
        }
    }
}