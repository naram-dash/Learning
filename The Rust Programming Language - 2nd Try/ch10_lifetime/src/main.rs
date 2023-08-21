fn main() {
    println!("Hello, world!");

    // compile time error
    // `x` does not live long enough
    /* {
        let mut r = &3;
        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    } */

    {
        let x = 5;
        let r = &x;
        println!("r: {}", r);
    }

    {
        let s1 = String::from("abc");
        let s2 = "xyz";

        let result = longest(s1.as_str(), s2);
        println!("the longest string is {}", result);
    }

    {
        let mut s = &String::new();
        let s2 = String::from("xyz");
        {
            let s1 = String::from("long string is long");
            let result = longest(s1.as_str(), s2.as_str());

            println!("the longest string is {}", result);

            // not works - borrow(lifetime) checker
            // s = &result.to_string();
        }

        println!("{}", *s)
    }
    {
        let mut s = &String::new();
        {
            let s1 = String::from("long string is long");
            let s2 = String::from("xyz");
            let result = longest(s1.as_str(), s2.as_str());

            println!("the longest string is {}", result);

            // works
            s = &result.to_string();
        }
    }

    {
        // novel String lifetime이 끝나는 안쪽으로 i가 무조건 끝나야함
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("could not find a  '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    // compile time error
    // {
    //     let mut i = {
    //         let novel = String::from("Call me Ishmael. Some years ago...");
    //         // `novel` does not live long enough
    //         // borrowed value does not live long enough
    //         let first_sentence = novel.split('.').next().expect("could not find a  '.'");

    //         ImportantExcerpt {part: first_sentence}
    //     };
    //     println!("kkk {}", i.part)
    // }

    {
        let s: &'static str = "i have a static lifetime";
    }
}

// compile time error
// 32 | fn longest(x: &str, y: &str) -> &str {
//     |               ----     ----     ^ expected named lifetime parameter
//     |
//     = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
//  help: consider introducing a named lifetime parameter
//     |
//  32 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     |           ++++     ++          ++          ++

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
    // let result = String::from("really long string");
    // result.as_str()
    // cannot return reference to local variable `result`
    // returns a reference to data owned by the current function
}


// 이 주석은 중요 부분의 인스턴스가 해당 부분 필드에 있는 참조보다 오래 지속될 수 없음을 의미합니다.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//explit lifetime parameter need here
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: Display,
{
    println!("Announcement!  {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}