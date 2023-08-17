mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}
        
        fn take_payment() {}
    }
}

// (pub use) pub use to re-exporting
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    // (pub use) call fn from re-exporting
    crate::eat_at_restaurant();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("wheat");

    // field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("apple");
    println!("i'd like to {} toast please", meal.toast);
}

fn deliver_order() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    order2.printHappy();
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order () {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Appetizer {
        pub fn printHappy(&self) {
            println!("happy {:?}", self)
        }
    }
}

