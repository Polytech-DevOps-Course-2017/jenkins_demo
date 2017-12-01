use std::ops;

/// a_plus_b is a generic function to add two things of the same type
pub fn a_plus_b<T: ops::Add<Output=T>>(a: T, b: T) -> T {
    a + b
}

#[cfg(test)]
mod tests {

    use ::a_plus_b;

    #[test]
    fn it_works() {
    }


    #[test]
    fn add_two_numbers_with_generic_function() {
        assert_eq!(a_plus_b(1, 4), 5);
    }
}
