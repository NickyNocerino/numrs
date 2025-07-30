mod arraynd;
pub use arraynd::*;

mod matrix;
pub use matrix::*;

mod space;
pub use space::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
