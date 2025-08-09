#[derive(Debug)]
pub enum TensorError{
    MissmatchedDimensions(String),
    DependencyError(String),
    DivideByZero,
}