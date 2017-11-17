#[cfg(test)]
mod tests {

    use std::ops;

    #[test]
    fn it_works() {
    }

    fn a_plus_b<T: ops::Add<Output=T>>(a: T, b: T) -> T {
        a + b
    }

    #[test]
    fn add_two_numbers_with_generic_function() {
        assert_eq!(a_plus_b(1, 4), 5);
    }
}
