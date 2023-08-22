use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
    {
        let mut store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );
    }

    {
        let expansive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        };

        {
            // declare function and closure inside running codes
            fn add_one_v1 (x: u32) -> u32 {x+1}
            let add_one_v2 = |x: u32| -> u32 {x + 1};

            // no type signature, compilation error
            // but ok in FN args
            // no arrow in closure
            // let add_one_v3 = |x| {x + 1};
            // let add_one_v4 = |x| x + 1;
        }
    }

    {
        // no automatic generic
        // automatically choose one concrete type
        let example_clousre = |x| x;

        // let s = example_clousre(String::from("hellow"));
        let n = example_clousre(5);
    }

    {
        // Closures can capture values from their environment in three ways, 
        // which directly map to the three ways a function can take a parameter: 
        // 1. borrowing immutably, 
        // 2. borrowing mutably, 
        // 3. and taking ownership. 
        // The closure will decide which of these to use based on 
        // what the body of the function does with the captured values.

        let list = vec![1,2,3];
        // borrow immutable ref
        println!("before defining closure {:?}", list);

        let only_borrows = || println!("from closure {:?}", list);
        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
    
        let mut borrows_mutably = || list.push(7);
    
        borrows_mutably();
        // returned mutable reference
        println!("After calling closure: {:?}", list);

        // Between the closure definition and the closure call, 
        // an immutable borrow to print isn’t allowed because no other borrows are allowed 
        // when there’s a mutable borrow. 

        // error if exists
        // borrowing mutable reference returned here
        // so we can't print list above
        // borrows_mutably();
    }

    {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // 소유권을 강제로 옮기려면 move 키워드를 앞에 붙인다
        // 메인 스레드와 새로만든 스레드는 서로 수명이 일치하지 않기 때문에
        // 이론적으로 불변참조의 lifetime이 invalid 하다
        // 그렇기 떄문에 아예 새로 만든 스레드 쪽으로 소유권을 주는 것을 강제한다.
        //
        // error[E0373]: closure may outlive the current function
        // 클로저가 현재 함수보다 더 오래 지속될 수 있다.
        thread::spawn(move || println!("From thread {:?}", list)).join().unwrap();
    }

    {
        // FnOnce() -> T
        // FnOnce << at least callable once 
        // () << no args
        // T << return type
    }

    {
        {
            let mut list = [
                Rectangle {width: 10, height: 1},
                Rectangle {width: 3, height: 5},
                Rectangle {width: 7, height: 12},
            ];

            let mut sort_operation = Vec::new() as Vec<String>;
            let value = String::from("by key called");

            let mut num_sort_operations = 0;
        
            list.sort_by_key(|r| {
                // because it is FnMut
                // allowed
                num_sort_operations += 1;
                // compile time error
                // ownership moved, so it can be called only once
                // FnOnce
                // but this is FnMut!!! (no move)!
                // sort_operation.push(value);
                r.width
            });
            // function signature
            // pub fn sort_by_key<K, F>(&mut self, mut f: F)
            // where
            // F: FnMut(&T) -> K,
            // K: Ord,
            //
            // borrow self mutable reference

        
            println!("{:#?}", num_sort_operations);
            println!("{:#?}", list);
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&mut self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}