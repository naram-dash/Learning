#[cfg(test)]
mod unit_test_included_in_normal_crate {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub fn add_two(a: i32) -> i32 {
    internal_addr(a, 2)
}

fn internal_addr(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod private_fn_tests {

    #[test]
    fn internal() {
        assert_eq!(4, super::internal_addr(2, 2));
        
    }
}