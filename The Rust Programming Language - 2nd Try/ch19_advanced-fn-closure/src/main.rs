fn main() {
    println!("Hello, world!");
{
    let answer = do_twice(add_one, 5);
}
    {
        let list_of_numbers = vec![1,2,3];
        let list_of_strings =
            //  list_of_numbers.iter().map(|i| i.to_string()).collect::<Vec<String>>();
            // ToString << trait 이다
             list_of_numbers.iter().map(ToString::to_string).collect::<Vec<String>>();
    }

    {
        enum Status {
            Value(u32),
            Stop,
        }
    
        let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }

    {
        fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
            Box::new(|x| x + 1)
        }
    }
}


fn add_one (x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// fn is function pointer
// fn is not trait like Fn, is Type
// so dont need generic