use rand::Rng;
use std::cmp::Ordering;
use std::io;
// use std::io::stdin;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..100);
    println!("the secret number is {secret_number}");
    loop {
        println!("Guess the number~!");
        println!("Please input your guess: ");

        // for i in 1..=100  {
        //     println!("{i}");
        // }

        // let apples = 5;
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // let guess = guess.trim().parse::<u32>().expect("Please type a number");

        let guess = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("****you win****");
                break;
            }
        };

        // let x = 5;
        // let y = 10;
        // println!("x = {x} and y + 2 = {}", y + 2);
    }
}
