#[derive(Clone, Copy, Debug)]
pub struct Quaternion{
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self {
            x:0.0,
            y:0.0,
            z:0.0,
            w:1.0,
        }
    }
}