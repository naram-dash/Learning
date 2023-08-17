use std::fs::File;
use std::error::Error;

// The main function may return any types that implement the std::process::Termination trait,
// which contains a function report that returns an ExitCode. 
fn main() -> Result<(), Box<dyn Error>> {
    // compile time error
    // let greeting_file = File::open("hello.txt")?;

    println!("{:?}", last_char_of_first_line("123"));
    println!("{:?}", last_char_of_first_line("abc"));
    println!("{:?}", last_char_of_first_line("가나다"));

    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
