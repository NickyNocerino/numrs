pub enum ArrayNDError{
    MissmatchedDimensions(String),

    IndexOutOfBounds(String),

    DivideByZero,


}