fn main() {
    println!("Hello, world!");

    // Matching Named Variables
    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end x = {:?}, y = {}", x, y);
    }

    // Multiple Pattern
    {
        let x = 1;

        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    // Matching Ranges of Values with ..=
    {
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    // destructing
    {
        struct Point {
            x: i32,
            y: i32,
        }

        let p = Point { x: 0, y: 7 };

        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        fn use_p_dest(Point { x, y }: Point) {
            println!("{} {} ", x, y);
        }

        use_p_dest(p);

        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {x}"),
            Point { x: 0, y } => println!("On the y axis at {y}"),
            Point { x, y } => {
                println!("On neither axis: ({x}, {y})");
            }
        }
    }

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
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }

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

        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        // nested pattern
        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

    {
        struct Point {
            x: i32,
            y: i32,
        }

        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    }

    // ignore pattern
    {
        fn foo(_: i32, y: i32) {
            println!("This code only uses the y parameter: {}", y);
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
                println!("Some numbers: {first}, {third}, {fifth}")
            }
        }
    }

    {
        let s = Some(String::from("Hello!"));

        if let Some(_s) = s {
            // can use _s
            println!("found a string {}", _s);
        }

        // moved
        // println!("{:?}", s);
    }

    {
        let s = Some(String::from("Hello!"));

        if let Some(_) = s {
            println!("found a string");
        }

        // but not moved with ignored
        println!("{:?}", s);
    }

    // ignore remaining part
    {
        struct Point {
            x: i32,
            y: i32,
            z: i32,
        }

        let origin = Point { x: 0, y: 0, z: 0 };

        match origin {
            Point { x, z, .. } => println!("x is {}", x),
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        // match numbers {
        //  명확하지 않은 패턴의 사용
        //  (.., second, ..) => {
        //      println!("Some numbers: {}", second)
        //  }
        // }
    }

    // match guard
    // 패턴만으로 표현하기 힘든 조건일 때 사용
    {
        let num = Some(4);

        match num {
            Some(x) if x % 2 == 0 => println!("The number {} is even", x),
            // if x % 2 == 0
            Some(x) => println!("The number {} is odd", x),
            None => (),
        }
    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            // 이 코드에서 y라고 써봐야 새 패턴으로 인식되서
            // 위 y는 10인데 여기서 패턴 일치 걸림
            // Some(y)=> println!("Matched, n = {y}"),

            // 이렇게 써야 맞음
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {:?}", x),
        }

        println!("at the end: x = {:?}, y = {y}", x);
    }

    {
        let x = 4;
        let y = false;
    
        match x {
            // (4 | 5 | 6) if y => .. 처럼 동작한다
            (4 | 5 | 6) if y => println!("yes"),
            // 이건 아예 돌지도 않음
            // 4 | 5 | (6 if y) => println!("yes"),
            _ => println!("no"),
        }
    }

    // @ at binding
    {
        enum Message {
            Hello { id: i32 },
        }
    
        let msg = Message::Hello { id: 5 };
    
        match msg {
            Message::Hello {
                // 하지만 @으로 패턴 테스트를 하면서 값으로 뺄 수 있다.
                id: id_variable @ 3..=7,
            } => println!("Found an id in range: {}", id_variable),

            // 일반 패턴을 걸었을때에는 변수로 못뺸다
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range")
            }
            Message::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}
