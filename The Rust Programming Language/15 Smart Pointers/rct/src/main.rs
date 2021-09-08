enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use std::rc::Rc;
use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))))));
    println!("a count : {} ", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after b : {} ", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after c : {} ", Rc::strong_count(&a));
    }
    println!("count after c dealloc: {} ", Rc::strong_count(&a));
}
