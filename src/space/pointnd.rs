use crate::*;

pub struct PointND {
    n: usize,
    coords: ArrayND<f64>,
}

impl PointND{
    pub fn origin(n: usize) -> Self{
        Self{
            n:n,
            coords: ArrayND::fill(vec![n], 0.0)
        }
    }
}