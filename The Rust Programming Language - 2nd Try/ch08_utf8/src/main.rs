use std::ops::Add;

fn main() {
    println!("Hello, world!");

    {
        let mut s = String::new();
    }

    {
        let data = "initial contents";

        let s = data.to_string();

        let s = "initial contents".to_string();

        let s = String::from("inital contents");
    }

    {
        let hello = String::from("السلام عليكم");
        let hello = String::from("Dobrý den");
        let hello = String::from("Hello");
        let hello = String::from("שָׁלוֹם");
        let hello = String::from("नमस्ते");
        let hello = String::from("こんにちは");
        let hello = String::from("안녕하세요");
        let hello = String::from("你好");
        let hello = String::from("Olá");
        let hello = String::from("Здравствуйте");
        let hello = String::from("Hola");
    }

    {
        let mut s = String::from("foo");

        s.push_str(" bar");
    }

    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);

        // s2 ownership alive
        println!("s2 is {s2}");
    }

    {
        let mut s = String::from("lo");
        s.push('l');
    }

    {
        let s1 = String::from("hello, ");
        let s2 = String::from("world");

        let s3 = s1 + &s2;
        // s1 ownership died
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // add take self ownership
        // let s = s1 + "-" + &s2 + "-" + &s3;

        // format! macro doesnt take ownership
        let s = format!("{s1}-{s2}-{s3}");
        let s = format!("{s1}-{s2}-{s3}");
    }

    {
        let s1 = String::from("hello");

        // String works like vec
        // but doesnt support indexing
        // let h = s1[0];
    }

    // A String is a wrapper over a Vec<u8>.
    {
        let hello = String::from("Hola"); // len = 4, 1char => 1byte
        let hello = String::from("Здравствуйте"); // len = 24,  1char => 2byte
                                                  // an index into the string’s bytes will not always correlate to a valid Unicode scalar value.
    }

    // UTF-8은
    // 가변 길이 인코딩 방식
    // https://hyoje420.tistory.com/3
    {
        let hello = "Здравствуйте";
        // compile time error
        // let answer = &hello[0];
    }

    // indexing operations are expected to always take constant time (O(1)).
    // But it isn’t possible to guarantee that performance with a String,
    // because Rust would have to walk through the contents from the beginning to the index
    // to determine how many valid characters there were.

    {
        let hello = "Здравствуйте";

        let s = &hello[0..4];
        // runtime error
        // let s = &hello[0..3];

        println!("{s}");
    }

    // use the chars method.
    {
        for c in "Зд".chars() {
            println!("{c}");
        }
        for c in "감사합니다".chars() {
            println!("{c}");
        }
        for b in "감사합니다".bytes() {
            println!("{b}");
        }
    }
}
