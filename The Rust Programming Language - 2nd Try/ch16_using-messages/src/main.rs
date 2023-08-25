use std::{sync::mpsc, thread, time::Duration, ops::Deref};

// mpsc
// ==> multi producer(tx), single consumer(rx)

fn main() {
    println!("Hello, world!");

    {
        // tx = Sender (다중 생산자) (transmit)
        // rx = Receiver (단일 소비자)
        let (sender, receiver) = mpsc::channel();

        // 생산자 소유권을 이동시킨다
        thread::spawn(move || {
            let val = String::from("hi");

            // 이 코드로 인해 전해지는 타입이 String으로 추론됨
            sender.send(val).unwrap();
        });

        // my own impletation
        // blocked
        // loop {
        //     let a = receiver.recv();
        //     if let Ok(msg) = a {
        //         println!("{msg}");
        //     } else {
        //         continue;
        //     }
        // }

        // book codes
        // let received = receiver.recv().unwrap();
        // println!("i got {}", received);
        
        // using try_recv
        // recv => 채널에서 값이 올때까지 기다림 (result => 메시지 왔으면 Ok : 통신 끊으면 Err)
        // try_recv => 기다리지 않고 result 바로 뱉음 (확인한 시점에서 result => 메시지가 있으면 Ok, 메시지가 없으면 Err)
        // 즉슨 loop 돌면서 메시지가 왔는지만 확인하면서, 메시지가 진짜 올때까지 멈추지 않고 딴거 하다가 정말 왔으면 하면 된다는거임. 
        // let received = receiver.try_recv().unwrap();
        // println!("i got {}", received);
    }


    {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();

            // compile time error
            // val ownership is moved out
            // println!("val is {}", val);
            // 소유권 만세! 만세! 만세!
        });

        let received = rx.recv().unwrap();
        println!("Got: {}", received);
    }

    {
        let (tx, rx) = mpsc::channel();

        let make_noise = move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            // for val in vals {
            //     tx.send(val).unwrap();
            // }

            vals.iter().for_each(|val| {
                // need to send with ownership 
                // if you send reference, then message's lifetime is matter to communicating each other
                tx.send(val.clone()).unwrap();
            });
        };

        thread::spawn(make_noise);

        // iterator using blocking 
        for received in rx {
            println!("Got: {}", received);
        }
    }

    {
        let (tx, rx) = mpsc::channel();

        let tx1 = tx.clone();

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });      

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });      

        for received in rx {
            println!("MPSC Got: {}", received);
        }
    }
}
