use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }
}


// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// above code is syntatic sugar for this(trait bound)
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// same function signature
pub fn notify2<T: Summary>(item1: &T, item2: &T) {}
pub fn notify3(item1: &impl Summary, item2: &impl Summary) {}

// impl multiple traits
pub fn notify4(item: &(impl Summary + Display)) {}
pub fn notify5<T: Summary + Display>(item: &T) {}


fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { 15 }
fn some_function2<T, U>(t: &T, u: &U) -> i32 
where 
    T: Display + Clone,
    U: Clone + Debug ,
{
    15
}


fn returns_summrizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// compile time error
// `if` and `else` have incompatible types

// because actual return type is multiple 
// (newsarticle, tweet)

// help: you could change the return type to be a boxed trait object
// | fn returns_summarizable2(switch: bool) -> Box<dyn Summary> {
//     help: if you change the return type to expect trait objects, box the returned expressions
//     |
// 89  ~         Box::new(NewsArticle {
// 90  |             headline: String::from(
//   ...
// 98  |             ),
// 99  ~         })
// 100 |     } else {
// 101 ~         Box::new(Tweet {
// 102 |             username: String::from("horse_ebooks"),
//   ...
// 107 |             retweet: false,
// 108 ~         })



/* fn returns_summarizable2(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
} */

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// trait bound conditionally
//  blanket implementation
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member in x = {}", self.x);
        } else {
            println!("the largest member in x = {}", self.x);
        }
    }
}