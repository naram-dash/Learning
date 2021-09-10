use std::thread;
use std::time::Duration;

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("새 스레드: {}", i);
//             thread::sleep(Duration::from_millis(1))
//         }
//     });
//     handle.join().unwrap();

//     for i in 1..5 {
//         println!("주 스레드: {}", i);
//         thread::sleep(Duration::from_millis(1))
//     }
// }
fn main() {
    let v = vec![1, 2, 3];
    let handle = std::thread::spawn(move || {
        println!("벡터: {:?}", v);
    });

    // drop(v);
    // manual drop
    // idontknow where v is dropped

    handle.join().unwrap();
}
