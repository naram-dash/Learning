use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");

    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi num {} from spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
    
        // method using self
        // handle.join().unwrap();
    
        for i in 1..5 {
            println!("hi num {} from main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    
        // 이게 없으니까 서브스레드에서 9th item까지 가지 않았는데 꺼져버림
        // handle.join().unwrap();
    }

    {
    //     let v = vec![1,2,3];

    //     let handle = thread::spawn(|| {
    //         // spawned thread에서 v를 빌려오려고(borrow) 하지만
    //         // 스레드 자체의 실행시간은 알수가없음
    //         // 그래서 참조로 가져오기도 힘듬
    //         println!("here's a vector: {:?}", v);
    //     });

    //    drop(v); // ? 에러 발생 코드
    //    handle.join().unwrap();
    }

    {
        let v = vec![1,2,3];

        // move를 통해 소유권을 옮김
        // 개념상으로 이제 메인스레드에서 v를 쓸 수 없다
        let handle = thread::spawn(move || {
            println!("here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
}
