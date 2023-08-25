use gui::{Draw, Screen};

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to acually draw select box
    }
}

fn main() {
    println!("Hello, world!");

    {
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ]
        };

        // 특성개체를 사용할때는 동적 디스패칭을 사용한다
        // 런타임 비용이 있다는 것
        // 하지만 그만큼 유연해졌잖아


        screen.run();
    }

    {
        // let screen = Screen {
        //     components: vec![Box::new(String::from("Hi"))],
        //     // compile time error
        //     // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
        // };

        // screen.run();
    }
}
