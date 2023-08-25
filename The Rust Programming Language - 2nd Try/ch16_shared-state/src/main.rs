use std::{sync::{Mutex, Arc}, thread, rc::Rc};

fn main() {
    println!("Hello, world!");

    {
        let m = Mutex::new(5);

        {
            // result type of lock method is LockResult
            // num is mutex guard
            let mut num = m.lock().unwrap();
            // change inside mutaxguard
            *num = 6;
        }

        println!("m = {:?}", m);
    }

    {
        // let counter = Rc::new(Mutex::new(0));
        

        // ARC is atomically reference counted
        // use mutex in arc
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            // let counter = Rc::clone(&counter);
            let counter = counter.clone();

            // send trait
            // 
            // Types that can be transferred across thread boundaries.
            //
            // This trait is automatically implemented when the compiler determines it's
            // appropriate.
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("Result: {}", *counter.lock().unwrap());
    }
}
