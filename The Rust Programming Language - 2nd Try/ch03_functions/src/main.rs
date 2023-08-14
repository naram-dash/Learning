fn main() {
    // another_function(5);
    print_labeled_measurement(5, 'c');

    // error!
    // let x = (let y = 6);

    let y = {
        let x = 3;
        x + 1
    };

    println!("the value of y is: {y}");

    let x = five();

    println!("the five is {x}");
    
    let x = plus_one(5);
    println!("the 5 + 1 is {x}");
}

fn another_function(x: i32) {
    // println!("another function")
    println!("the value of x is: {x}")
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("the measurement is: {value} {unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}