use std::rc::Rc;
fn main() {
    println!("Hello, world!");
    
    {
        enum List {
            Cons(i32, Box<List>),
            Nil
        }
        use List::{Cons, Nil};
        
        let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        let b = Cons(3, Box::new(a));
        // ownership moved!
        // compile time error
        // let c = Cons(3, Box::new(a));
    }

    {
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, a.clone());
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(3, a.clone());
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after drop c = {}", Rc::strong_count(&a));

    }
}
