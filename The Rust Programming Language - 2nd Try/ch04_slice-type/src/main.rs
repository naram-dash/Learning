use std::{string, io::Bytes};

fn main() {
    println!("Hello, world!");

    {
        let mut s = String::from("hello world");

        let word = first_word(&s);

        s.clear();
    }

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        let slice = &s[0..2];
        let slice = &s[..2];
    }
    {
        let s = String::from("hello");
        let len = s.len();

        let slice = &s[3..len];
        let slice =  &s[3..];
    }

    {
        let s = String::from("hello");

        let len = s.len();

        let slice = &s[0..len];
        let slice = &s[..];
    }

    {
        let mut s = String::from("hello world");

        let word = first_word(&s);
        // String::clear(&mut s);
        // s.clear();

        println!("the first word is {}", word);
    }

    {
        let s = "hello world";
    }

    {
        let my_string = String::from("hello world");

        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);

        let word = first_word(&my_string);

        let my_string_literal = "hello world";

        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
        let word = first_word(my_string_literal);
    }

    {
        let a = [1,2,3,4,5];

        let slice = &a[1..3];
        assert_eq!(slice, &[2,3]);
        assert_eq!(slice, &[2,3,4]);
    }
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn second_word(s: &String) -> (usize, usize) {
    (0, 0)
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// fn second_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     bytes.iter().enumerate().
// }