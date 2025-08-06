use crate::*;

#[derive(Clone)]
pub struct PointND {
    coords: Vec<f64>,
}

impl From<VectorND> for PointND {
    fn from(item: VectorND) -> Self {
        Self {
            coords: item.dir_vec(),
        }
    }
}

impl PointND{
    pub fn origin(n: usize) -> Self{
        Self{
            coords: vec![0.0;n]
        }
    }

    pub fn coords_vec(&self) -> Vec<f64> {
        self.coords.clone()
    }
}

