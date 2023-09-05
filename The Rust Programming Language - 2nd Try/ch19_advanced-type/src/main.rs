fn main() {
    println!("Hello, world!");

    {
        type Kilometers = i32;

        let x: i32 = 5;
        let y: Kilometers = 5;

        println!("x + y = {}", x + y);
    }

    // {
    //     let guess: i32 = match " 123 ".trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     }
    // }

    // {
    //     print!("forever ");

    //     loop {
    //         print!("and ever ");
    //     }
    // }

    {
        // use &str than str, you can't know size of str in runtime
        // let s1: str = "Hello there!";
        // let s2: str = "How's it going?";
    }
}

use std::fmt;
use std::io::Error;

type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    fn flush(&mut self) -> Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn flush_all(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

fn bar() -> ! {
    panic!("123")
}

// impl<T> Option<T> {
//     pub fn unwrap(self) -> T {
//         match self {
//             Some(val) => val,
//             None => panic!("called `Option::unwrap()` on a `None` value"),
//         }
//     }
// }

fn generic<T>(t: T) {
    // --snip--
}

// is same

fn generic2<T: Sized>(t: T) {
    // --snip--
}

// Size trait 완화하기
// ?trait는 size trait 에서만 적용가능
// 포인터에다가 사용해야함

fn generic3<T: ?Sized> (t: &T) {

}