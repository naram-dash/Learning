use std::net::{IpAddr as StdIpAddr, Ipv4Addr as StdIpv4Addr, Ipv6Addr as StdIpv6Addr};

fn main() {
    println!("Hello, world!");

    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;

        route(four);
        route(six);

        // let home = IpAddr {
        //     kind: IpAddrKind::V4,
        //     address: String::from("127.0.0.1")
        // };

        // let loopback = IpAddr {
        //     kind: IpAddrKind::V6,
        //     address: String::from("::1")
        // };
    }

    // {
    //     let home = IpAddr::V4(String::from("127.0.0.1"));
    //     let loopback = IpAddr::V6(String::from("::1"));
    // }
    {
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
    }

    {
        let realV4 = StdIpv4Addr::new(127, 0, 0, 1);
    }

    {
        let a = Message::Quit;

        let m = Message::Write(String::from("hello"));
        m.call();
    }

    {
        let some_number = Some(5);
        let some_char = Some('e');

        let absent_number: Option<i32> = None;

        let x: i8 = 5;
        let y: Option<i8> = Some(5);

        if let Some(yValue) = y  {
            let sum = x + yValue;

            print!("what is sum: {}", sum);
        }

    }
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

// enum IpAddr {
//     V4(String),
//     V6(String)
// }
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
