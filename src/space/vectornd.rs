use crate::*;

#[derive(Clone)]
pub struct VectorND {
    dir: Vec<f64>,
}

impl From<PointND> for VectorND {
    fn from(item: PointND) -> Self {
        Self {
            dir: item.coords_vec(),
        }
    }
}

impl VectorND {
    pub fn normalized(&self) -> Self {
        let mut new = Vec::<f64>::new();
        let magnitude = self.dir.iter().map(|x| x*x ).sum::<f64>().sqrt();
        for value in self.dir.clone() {
            new.push(value / magnitude);
        }
        Self {
            dir: new,
        }
    }

    pub fn dir_vec(&self) -> Vec<f64> {
        self.dir.clone()
    }
}

