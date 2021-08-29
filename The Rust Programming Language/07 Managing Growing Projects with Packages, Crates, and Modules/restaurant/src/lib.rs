// #[cfg(test)]

// pub fn eat_at_restaurant() {
//     // 절대 경로
//     crate::front_of_house::hosting::add_to_waitlist();
//     // 상대 경로
//     front_of_house::hosting::add_to_waitlist();
// }

// 루트 라이브러리 크레이트에서 모듈이 있음을 확인
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn hi() {
    crate::eat_at_restaurant();
}

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use self::front_of_house::hosting;
// use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // 비공개 필드에 접근할 수 없다.
    // meal.seasonal_fruit = String::from("블루베리");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult;

// fn function1() -> Result {
//     Result::Ok(());
// }
// fn function2() -> IoResult<()> {
//     IoResult::Ok(());
// }

// pub use crate::front_of_house::hosting;
