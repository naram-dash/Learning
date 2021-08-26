const MAX_POINTS: u32 = 100_000;

// fn main() {
//     let mut x = 5;
//     println!("x: {}", x);
//     x = 6;
//     println!("x: {}", x);
// }

fn main() {
    let x = 5;
    println!("x: {}", x);
    let x = 6;
    println!("x: {}", x);

    let x = 7;
    println!("x: {}", x);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    println!("a: {}", a[0]);

    another_function();
}

fn another_function() {
    println!("또 다른 함수🎠🎫🎫🎟🎞🎗🎀🎍🎍🎍");
}
