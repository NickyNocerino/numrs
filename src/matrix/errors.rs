#[derive(Debug)]
pub enum MatrixError{
    MissmatchedDimensions(String),
    DivideByZero,
}