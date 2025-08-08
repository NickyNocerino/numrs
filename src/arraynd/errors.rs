pub enum ArrayNDError{
    MissmatchedDimensions(String),

    IndexOutOfBounds(String),

    IllegalSlice(String),

    DivideByZero,


}