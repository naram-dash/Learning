fn main() {
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let x = 5;

    let x = x + 1;

    let x = {
        let x = x * 2;
        println!("{x}");
        x
    };
    
    println!("{x}");


    let spaces = "     ";
    let spaces = spaces.len();
    
}
