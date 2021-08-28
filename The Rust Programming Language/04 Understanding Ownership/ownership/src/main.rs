fn main() {
    // move
    {
        let s: &str = "hello";
        let s2: String = String::from("hello");
    }
    {
        let mut s = String::from("hello");
        s.push_str(", world");
        println!("{}", s);
    }

    {
        let x = 5;
        let y = x;
    }
    {
        let mut s = String::new();
        {
            let s1 = String::from("sioejfsioefjoisef");
            s = s1;
        }
        println!("{}", s);
    }
    {
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}, world", s1);
        println!("{}, world", s2);
    }

    // clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();
        println!("{}, world", s1);
        println!("{}, world", s2);
    }

    {
        let s = String::from("hello");

        takes_ownership(s);

        // value used here after move
        // takes_ownership(s);

        let x = 5;

        makes_copy(x);

        makes_copy(x);

        fn takes_ownership(some_string: String) {
            println!("{}", some_string);
        }

        fn makes_copy(some_integer: i32) {
            println!("{}", some_integer);
        }
    }

    {
        let s1 = gives_ownership();

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);

        fn gives_ownership() -> String {
            let some_string = String::from("hello");
            some_string
        }

        fn takes_and_gives_back(a_string: String) -> String {
            a_string
        }
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("{}의 길이는 {}입니다", s2, len);

        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len();

            (s, length)
        }
    }

    {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }

    {
        let s = String::from("hello");
        change(&s);

        fn change(some_string: &String) {
            // 수정못한다.
            // some_string.push_str(", world");
        }
    }
    {
        let mut s = String::from("hello");
        change(&mut s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }
    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        let r2 = &mut s;
        // println!("{} {}", r1, r2);
    }

    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
            println!("{}", r1);
        }

        let r2 = &mut s;
        println!("{}", r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        // let r3 = &mut s;
        // 가변참조가 있으면 참조 개수는 1개로 제한된다.
        println!("{} {}", r1, r2);
    }

    {
        let reference_to_nothing = dangle();

        fn dangle() -> String {
            let s = String::from("hello");

            // &s
            // 이때는 그냥 소유권 이동하자
            s
        }
    }

    {
        let mut s = String::from("hello world");
        let word = first_word(&s);
        s.clear();
        fn first_word(s: &String) -> usize {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return i;
                }
            }

            s.len()
        }
    }

    {
        let s = String::from("hello world");

        let hello = &s[0..5];
        let world = &s[6..11];

        println!("{}{}", hello, world);
        println!("{}{}", &hello, &world);
    }

    {
        fn first_word(s: &String) -> &str {
            // fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let s = String::from("hello world");

        let word = first_word(&s);

        // s.clear();

        println!("the f word = {}", word);
    }

    {
        fn first_word(s: &str) -> &str {
            // fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let my_string = String::from("hello world");

        // 둘다 됨
        // let word = first_word(&my_string[..]);
        let word = first_word(&my_string);

        let my_string_literal = "hello world";
        let word = first_word(&my_string_literal);
        let word = first_word(my_string_literal);
    }

    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
    }
}
