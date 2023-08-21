#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn another() {
        // panic!("make this test fail");
        assert_eq!(3, 4);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
        // self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod rect_test {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 3
    // a + 2
}

#[cfg(test)]
mod add_test {
    use super::*;

    #[test]
    fn if_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

pub fn greeting(name: &str) -> String {
    // format!("Hello {}!", name)
    String::from("hello!")
}

#[cfg(test)]
mod greet_test {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("carol");
        assert!(
            result.contains("carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
        // if value < 1 || value > 100 {
            panic!("Guess value must be ge than 1, got {}.", value);
        } else if value > 100 {
            panic!("Guess value must be le then 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod guess_test {
    use super::*;

    #[test]
    #[should_panic(expected = "le then 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

#[cfg(test)]
mod result_test {
    #[test]
    // Err를 리턴하는 것은 test case fail로 여겨진다
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 3 {
        // if 2 + 2 == 4 {
            Ok(())
        }else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}