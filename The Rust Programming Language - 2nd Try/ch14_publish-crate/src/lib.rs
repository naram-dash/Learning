//! # Art
//! 
//! A lib for modeling artistic concepts.

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red, 
        Yellow,
        Blue
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Blue
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Blue
    }
}



/// ! # My crate
/// ! 
/// ! `my_crate` is a collection of utilties to makeperfomaing ertain
/// ! calculation more convienent
/// ! 맨처음에 나와야함


/// adds one to the number given
/// 
/// # Examples
/// cargo test에서 예제코드도 같이 테스트된다.
/// 
/// 
/// ```
/// let arg = 5;
/// let answer = ch14_publish-crate::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}



