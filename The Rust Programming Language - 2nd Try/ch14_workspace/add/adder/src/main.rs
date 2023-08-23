use add_one;
// use rand;
// you cant you rand if each package (not workspace) depends on it

// cargo run -p adder
// -p is --package
fn main() {
    let num = 10;
    println!("hello world {num} plus one is {}",add_one::add_one(num));
}
