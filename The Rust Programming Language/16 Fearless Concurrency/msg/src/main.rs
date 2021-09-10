use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// simple msg
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         // println!("val = {}", val); error;
//     });

//     let received = rx.recv().unwrap();
//     println!("rcv! {}", received);
// }

// multiple msg
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("baby thread"),
            String::from("said"),
            String::from("hie"),
            String::from("!"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("AND"),
            String::from("MORE"),
            String::from("MESSAGE"),
            String::from("HERE!"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("수신: {}", received);
    }
}
