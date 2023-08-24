use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None
        }
    }
}

fn main() {
    println!("Hello, world!");

    {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a)); // 1
        println!("a next item = {:?}", a.tail()); // Some(RefCell {value: Nil})
    
        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
        println!("a rc count after b creation = {}", Rc::strong_count(&a)); // 2
        println!("b initial rc count = {}", Rc::strong_count(&b)); // 1
        println!("b next item = {:?}", b.tail()); // Some(RefCell { value: Cons(5, RefCell { value: Nil })})
    
        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
    
        println!("b rc count after changing a = {}", Rc::strong_count(&b)); // 2
        println!("a rc count after changing a = {}", Rc::strong_count(&a)); // 2
        // become Circualr dependancy
    
        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // runtime error (overflow)
        // println!("a next item = {:?}", a.tail());
    }

    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>
        }

        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        
        {
            let branch = Rc::new(Node {
                value: 5,
                parent: RefCell::new(Weak::new()),
                children: RefCell::new(vec![Rc::clone(&leaf)])
            });
    
            // 나중에 parent 추가 
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );
            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }
        
        // strong count가 0이 될 때  weak count는 몇개가 있던 0가 되어버린다. (None)
        println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );

    }
}
