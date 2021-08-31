use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

// fn main() {
//     println!("Hello, world!");
//     panic!("crash and burn");
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     let one_of = v[99];
// }

// use std::fs::File;
// use std::io::ErrorKind;

// fn main() {
//     // f는 Result 타입
//     let f = File::open("hello.txt");

//     let f = match f {
//         Ok(file) => file,
//         // Returns the corresponding [ErrorKind] for this error.
//         Err(ref error) => match error.kind() {
//             ErrorKind::NotFound => match File::create("hello.txt") {
//                 Ok(fc) => fc,
//                 Err(e) => panic!("파일을 생성하지 못함 {}", e),
//             },
//             other_error => panic!("파일을 열지 못함! {:?}", other_error),
//         },
//     };
// }

// use std::fs::File;

// fn main() {
//     // Err면 패닉
//     // let f = File::open("hello.txt").unwrap();
//     // panic 에러 메시지
//     // let f = File::open("hello.txt").expect("error in open files");
//     let f = File::open("hello.txt")?;
// }

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}

// fn read_username_from_file() -> Result<String, io::Error> {
//     let f = File::open("hello.txt");

//     let mut f = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s = String::new();

//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut s = String::new();
//     File::open("hello.txt")?.read_to_string(&mut s)?;
//     Ok(s)
// }
fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
