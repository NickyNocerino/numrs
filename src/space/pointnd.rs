use crate::*;

pub struct PointND {
    coords: Vec<f64>,
}

impl PointND{
    pub fn origin(n: usize) -> Self{
        Self{
            coords: vec![0.0;n]
        }
    }
}