use crate::*;

#[derive(Clone, Debug)]
pub struct RayND {
    origin: PointND,
    dir:VectorND,
}

impl RayND {
    pub fn new(origin: &PointND, dir: &VectorND) -> Self {
        Self {
            origin: origin.clone(),
            dir: dir.normalized(),
        }
    }
}