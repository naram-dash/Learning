// fn main() {
//     let r;

//     {
//         let x = 5;
//         r = &x; // 대입하는 시점에서 오류!
//         println!("r: {}", r);
//     }

//     println!("Hello, world!");
// }

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목하라 {}", announcement);
        self.part
    }
}

fn main() {
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("더 긴 문자열: {}", result);
    }

    {
        let string1 = String::from("아주 아주 긴 문자열");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("{}", result);
        }
    }

    // {
    //     let string1 = String::from("아주 아주 긴 문자열");
    //     let result;
    //     {
    //         let string2 = String::from("xyz");
    //         result = longest(string1.as_str(), string2.as_str());
    //     }
    //     // 아무리 실행 결과가 정해져있더라도 rust는 알 수 없다.
    //     println!("{}", result);
    // }

    {
        let string1 = String::from("아주 아주 긴 문자열");
        let result;
        {
            let string2 = String::from("xyz");
            result = get_first(string1.as_str(), string2.as_str());
        }
        println!("{}", result);
    }

    {
        let novel = String::from("star wars. 오래전 === .");
        // &str
        let first_sentence = novel
            .split('.')
            .next()
            .expect("문장에서 마침표 . 를 찾을수 없습니다.");

        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    {
        // 리터럴은 프로그램 바이너리에 저장되므로 전체 수명이다.
        let s: &'static str = "문자열은 정적 수명이다.";
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("shishisifjseoifejs");
//     // 'a 가 매개변수와 관련되지 않음
//     // 그래서 함수가 끝날때 소유권도 끝남
//     result.as_str()
// }

fn get_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// fn get_first2(x: &str, y: &str) -> &str {
//     x
// 2번(매개변수가 1개), 3번(메소드) 적용안됨
// 그래서 컴파일러가 수명을 추론할 수 없음
// }

// fn get_number<'a>(x: &'a str) -> &'a u32 {
//     // _dangling_reference
//     let result = 24;
//     &result
// 안됌
// }

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

use std::fmt::Display;

fn longest_with_an_annoucement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("주목: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
