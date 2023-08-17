mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}
        
//         fn take_payment() {}
//     }
// }

// use crate::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_waitlist();

//     front_of_house::hosting::add_to_waitlist();

//     hosting::add_to_waitlist();

//     let mut meal = back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("wheat");

//     // field `seasonal_fruit` of struct `Breakfast` is private
//     // meal.seasonal_fruit = String::from("apple");
//     println!("i'd like to {} toast please", meal.toast);
// }

// fn deliver_order() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;

//     order2.printHappy();
// }

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::deliver_order();
//     }

//     fn cook_order () {}

//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
//         }
//     }

//     #[derive(Debug)]
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     impl Appetizer {
//         pub fn printHappy(&self) {
//             println!("happy {:?}", self)
//         }
//     }
// }


// mod customer {
//     pub fn eat_at_restaurant() {
//         // 코드 최상단에 use로 가져왔지만 안쪽까지 가져오지는 못한다
//         // 해당 scope에서만 사용 가능
//         // hosting::add_to_waitlist();

//         use crate::front_of_house::hosting;
//         hosting::add_to_waitlist();
//     }
// }

// // use std::cmp::Ordering;
// // use std::io;
// // use std::{cmp::Ordering, io};

// // use std::io;
// // use std::io::Write;

// use std::io::{self, Write};

// use std::collections::*;

// use dontuse_this_001::mix;

