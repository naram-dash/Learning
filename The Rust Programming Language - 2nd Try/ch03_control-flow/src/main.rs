fn main() {
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // error
    // if number {
    //     println!("number was exist?");
    // };

    if number != 0 {
        println!("not zero")
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {5} else {6};
    // let number = if condition {5} else {"siz"};

    println!("the value of number: {number}");

    // loop {
    //     println!("again!");
    // }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result of counter is {result}");

    let mut counter = 0;
    'counting_up : loop {
        println!("count = {counter}");
        let mut remaining = 10;
        
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            } 
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }

    println!("end count is {counter}");

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;

    }

    println!("LIFT OFF!!!");

    let a = [10,20,30,40,50,];
    let mut index = 0;
    while index < a.len() {
        println!("the value is {} ", a[index]);
        index +=1;
    }

    for element in a {
        println!("the value is in for {element}");
    }

    
    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("L:IFTOFF!!!");
}
