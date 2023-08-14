fn main() {
    let x = 2.0;

    let y: f32 = 3.0;

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let production = 4 * 30;

    let quotient = 56.7 / 32.2;

    let truncated = -5 / 3;

    let remainder = 43 % 5;

    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '💥';

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of y is  {y}");

    let five_hundred = tup.0;
    let six_point_one = tup.1;
    let one = tup.2;

    let a = [1,2,3,4,5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1,1,1,1,1,];

    let a = [3;5];

    let first = a[0];

    let second = a[1];

    // panic! here
    let invalid = a[10];

}
