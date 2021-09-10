//! # wow this is amzing
//! wowow

// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
// Atomic Rc

// // simple mutex
// fn main() {
//     let m = Mutex::new(5);

//     {
//         let mut num = m.lock().unwrap();
//         println!("{}", num);
//         *num = 6;
//     }
//     println!("m = {:?}", m);
// }

// multi thread mutex
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }
    // 1부터 10까지의 스레드가 비동기적으로 실행된다고 보면 된다.

    for handle in handles {
        handle.join().unwrap();
    }

    println!("result: {}", *counter.lock().unwrap());
}
