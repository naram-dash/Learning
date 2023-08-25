fn main() {
    println!("Hello, world!");

    {
        // let some_option_value = Some(1);
        // // this is refutable(반박 가능한) pattern
        // // because can check error in compile time
        // let Some(x): Option<i32> = some_option_value; // Not ok 
    }

    {
        let some_option_value = Some(1);
        if let Some(x) = some_option_value {
            println!("{}" ,x);
        }




        // below cases are irrefutable patterns
        if let x = some_option_value {
            // this is ok
        }
        if let x = 5 {
            println!("{}", x);
        };
    }
}
