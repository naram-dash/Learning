fn main() {
    println!("Hello, world!");

    {
        // is heap or stack? ==> not heap, it just stacked value by returned
        #[derive(Debug)]
        struct MyStruct {
            i: i32,
            s: String,
        }
        
        fn return_is_just_return() -> MyStruct {
            let mut my_struct = MyStruct { i: 23, s: String::from("123") };

            // println!("{:?}", &my_struct);
        
            my_struct
            // just value returned
        }

        let a = return_is_just_return(); // returned value
        let b = MyStruct {i: 12, s: String::from("89u")}; // stack value
        // println!("a: {:#?}, b: {:#?}", a, b);
    }


    {
        let b = Box::new(5);

        println!("b = {}", b);
    }

    {
        // error[E0072]: recursive type `List` has infinite size
        // enum List {
        //     Cons(i32, List),
        //     Nil
        // }

        enum List {
            Cons(i32, Box<List>),
            Nil
        }
        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }

    {
        fn return_box () -> Box<i32> {
            let a = Box::new(3875);
            a
        }

        
        let b = return_box();

        println!("box has ==> {}", b);
    }
}



