enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quater(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quater(state) => {
            println!("State quater {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("Hello, world!");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    // let home = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(6);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    {
        let some_u8_value = 0u8;

        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => println!("one"),
        }
    }
    {
        let some_u8_value = Some(0u8);

        // match some_u8_value {
        //     Some(3) => println!("three"),
        //     _ => (),
        // }

        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
    {
        let mut count = 0;
        if let Coin::Quater(state) = Coin::Quater(UsState::Alabama) {
            println!("{:?}주의 25센트 동전", state);
        } else {
            count += 1;
        }
    }
}
