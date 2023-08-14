fn main() {
    {
        let s1 = String::from("hello");

        let len = calculate_length(&s1);

        println!("the length of {} is {}", s1, len);
    }

    {
        let mut s = String::from("hello");
        change(&mut s);

        println!("this is {}", s);
    }

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        // error, no two mut ref
        // let r2 = &mut s;
        // println!("{}, {}", r1, r2);
        // println!("{}, {}", s, r1);
        // println!("{}", s);
        println!("{}", r1);
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;

        }// r1 goes out of scope here, so we can make a new reference with no problems.
        let r2 = &mut s;
    }

    {
        let mut s = String::from("hello");

        let r1 = &s; // no problem
        let r2 = &s; // no problem
        // let r3 = &mut s; // BIG PROBLEM
        //  cannot borrow `s` as mutable because it is also borrowed as immutable
        // 이미 immut ref를 만들었으니 그다음에 mut ref를 만들수가 없음
        // 그리고 한 시점에서 mut ref, immut ref 2개를 동시에 사용
        // mut ref는 2개 이상의 ref가 존재할 수 없음
    
        // println!("{}, {}, and {}", r1, r2, r3);
    
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        println!("{} and {}", r1, r2);

        // 한 시점에 ref가 두개이상 존재하지 않으니 가능!!!
        let r3 = &mut s;
        println!("{}", r3);
    }

    {
        // let reference_to_nothing = dangle();
    }
    
    {
        let reference_to_nothing = no_dangle();
    }
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// not working
// fn change(some_string: &String) {
//     some_string.push_str("123");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String {
//     //^ expected named lifetime parameter
//     let s = String::from("hello");

//     &s
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}