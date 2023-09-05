use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("hello, macro! my name is pancakes");
    }
}


fn main() {
    println!("Hello, world!");

    {
        let v = vec![1,2,3];
    }

    {
        Pancakes::hello_macro();
    }
}

