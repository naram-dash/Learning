fn five() -> i32 {
    5;
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// HELLO

fn main() {
    // println!("Hello, world!");
    // another_function(5, 3);

    // let x = 5;

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // let x = five();
    let x = plus_one(4);
    println!("x의 값: {}", x);
}
fn another_function(x: i32, y: i32) {
    println!("x의 값: {}", x);
    println!("y의 값: {}", y);
}
