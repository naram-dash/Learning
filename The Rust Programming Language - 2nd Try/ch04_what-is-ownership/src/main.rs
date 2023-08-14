fn main() {
    {
        let s = "hello";
    }

    {
        let s = String::from("hello");
    }

    {
        let mut s = String::from("hello");

        s.push_str(", world");

        println!("{s}");
    }

    {
        let x = 5;
        let y = x;

        let s1 = String::from("hello");
        let s2 = s1;

        // error
        // println!("{}, world", s1);
    }

    {
        let s1 = String::from("hello");
        let mut s2 = s1.clone();

        s2.push_str("worldworld");

        println!("s1 = {}, s2 = {}", s1, s2);
    }

    {
        let x = 5;
        let y = x;
    
        println!("x = {}, y = {}", x, y);
    
    }

    {
        let s = String::from("hello");
        takes_ownership(s);

        // error
        // println!("{}", s);

        let x = 5;
        makes_copy(x);

        println!("{}", x);
    }

    {
        let s1 = give_ownership();

        let s2 = String::from(s1);

        let s3 = takes_and_gives_back(s2);
    }

    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("the length of {}, is {}", s2, len)
    }
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}
fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}