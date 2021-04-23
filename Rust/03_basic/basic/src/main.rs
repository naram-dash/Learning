// // println!
// fn main() {
//     let a_number = 10;
//     let a_boolean = true;
//     println!("the number is {}.", a_number);
//     println!("the boolean is {}.", a_boolean);
// }

// // mut
// fn main() {
//     let mut a_number = 10;
//     println!("the number is {}.", a_number);
    
//     a_number = 15;
//     println!("the number is {}.", a_number);
// }

// shadowing
// fn main() {
//     let number = 5;
//     let number = number + 5;
//     let number = number * 2;
//     println!("The number is: {}", number)
// }

// data type
// fn main () {
//     let number: u32 = "42".parse().expect("Not a number!");
// }

// real number
// fn main () {
//     let x = 2.0; // f64 by default
//     let y: f32 = 3.0; // f32 via type annotation
// }

// calculate
// fn main() {
//     // Addition
//     println!("1 + 2 = {}", 1u32 + 2);

//     // Subtraction
//     println!("1 - 2 = {}", 1i32 - 2);
//     // ^ Try changing `1i32` to `1u32` to see why the type is important

//     // Integer Division
//     println!("9 / 2 = {}", 9u32 / 2);

//     // Float Division
//     println!("9 / 2 = {}", 9.0 / 2.0);

//     // Multiplication
//     println!("3 * 6 = {}", 3 * 6)
// }

// bool
// fn main () {
//     let is_bigger = 1 > 4;
//     println!("{}", is_bigger);
// }

// char
// fn main () {
//     let c = 'z'; // 32bit unicode
//     let z = 'ℤ';
//     let heart_eyed_cat = '😻';
// }

// String 
// literal string is always '&str'
// fn main () {
//     let mut hello = String::from("Hello, ");
//     hello.push('w');
//     hello.push_str("orld!");
//     println!("{}", hello)
// }

// tuple indexing
fn main () {
    let tuple = ("hello", 5, 'c');

    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
}