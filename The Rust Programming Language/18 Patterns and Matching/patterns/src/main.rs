fn main() {
    // if let condition with general if else
    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();
        if let Some(color) = favorite_color {
            println!("favorite color: {}", color);
        } else if is_tuesday {
            println!("green");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("purple!");
            } else {
                println!("orange")
            }
        } else {
            println!("blue")
        }
    }

    // while let condition loop
    {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // for loop
    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("index {}: {}", index, value);
        }
    }

    // also, let is pattern
    {
        let (x, y, z) = (1, 2, 3);
        // let (x, y) = (1, 2, 3); // error
        let (x, y, _) = (1, 2, 3);
    }

    // function pattern
    {
        fn foo(x: i32) {
            // code here
        }

        // deconstruct tuple in argv
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("pos: ({}, {})", x, y);
        }

        let point = (3, 5);
        print_coordinates(&point);
    }

    // refusable and irrefusable
    {
        // irrefusable
        let some_option_value = Some(5);
        // let Some(x) = some_option_value; // error

        // refusable
        if let Some(x) = some_option_value {
            println!("{}", x);
        }

        // warning: irrefutable `if let` pattern
        // this pattern will always match, so the `if let` is useless
        if let x = 5 {
            println!("{}", x);
        }
    }

    // pattern grammers

    {
        let x = 5;

        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("else"),
        }
    }

    // named variable
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("50"),
            Some(y) => println!("same, y = {:?}", y),
            _ => println!("not same, x = {:?}", x),
        }

        println!("result, x = {:?}, y = {:?}", x, y);
    }

    // or pattern
    {
        let x = 1;

        match x {
            1 | 2 => println!("1 or 2"),
            3 => println!("3"),
            _ => println!("else..."),
        }
    }

    // ... range
    {
        let x = 5;
        // let v = vec![1...5]; // not 3 dots in vec literal

        match x {
            // include 5
            1...5 => println!("num in 1 to 5"),
            _ => println!("else..."),
        }

        let c = 'c';

        match c {
            'a'...'j' => println!("ASCII 1/2"),
            'k'...'z' => println!("ASCII 2/2"),
            _ => println!("else..."),
        }
    }

    // divide and deconstruct
    struct Point {
        x: i32,
        y: i32,
    }
    {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        assert_eq!(p.x, a);
        assert_eq!(p.y, b);

        // same field name
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;
        assert_eq!(p.x, x);
        assert_eq!(p.y, y);
    }

    // match struct
    {
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("on line x, x: {}", x),
            Point { x: 0, y } => println!("on line y, y: {}", y),
            Point { x, y } => println!("not on line {}, {}", x, y),
        }
    }

    // deconstruct enum
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("Quit: no construct");
            }
            Message::Move { x, y } => {
                println!("move: {},{}", x, y);
            }
            Message::Write(s) => {
                println!("write: {}", s);
            }
            Message::ChangeColor(r, g, b) => {
                println!("r: {} g: {} b: {}", r, g, b);
            }
        }
    }

    // nested deconstructing
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }

        let msg = Message::ChangeColor(Color::Hsv(0, 113, 245));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("r: {} g: {} b: {}", r, g, b);
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("h: {} s: {} v: {}", h, s, v);
            }
            _ => {}
        }
    }

    // nested struct and tuple
    {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    {
        fn foo(_: i32, y: i32) {
            println!("only y : {}", y);
        }

        foo(3, 4);
    }

    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing customized value");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {}, {}, {}", first, third, fifth)
            }
        }
    }

    // unused code wraning disable
    {
        let _x = 5;
        let y = 10;
    }

    {
        let s = Some(String::from("Hello!"));

        // _s를 빼가면서 소유권을 가져감
        // if let Some(_s) = s {
        //     println!("found a string");
        // }

        // _는 바인딩하지 않으므로 소유권이 이동하지 않음
        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, .. } => println!("x is {}", x),
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {}, {}", first, last);
            }
        }
    }

    {
        // 모호한 범위
        // let numbers = (2, 4, 8, 16, 32);

        // match numbers {
        //     (.., second, ..) => {
        //         println!("Some numbers: {}", second)
        //     }
        // }
    }

    // match guard
    {
        let num = Some(4);

        match num {
            Some(x) if x < 5 => println!("less than 5, {}", x),
            Some(x) => println!("{}", x),
            None => (),
        }
    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {:?}", n),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {:?}", x, y);
    }

    {
        let x = 4;
        let y = false;

        match x {
            // y는 bool 타입
            4 | 5 | 6 if y => println!("yes"),
            _ => println!("no"),
        }
    }

    {
        enum Message {
            Hello { id: i32 },
        }

        let msg = Message::Hello { id: 5 };

        match msg {
            Message::Hello {
                id: id_variable @ 3...7,
            } => {
                println!("Found an id in range: {}", id_variable)
            }
            Message::Hello { id: 10...12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id)
            }
        }
    }
}
