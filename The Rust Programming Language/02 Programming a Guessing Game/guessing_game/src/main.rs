use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("guess number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("맞춰야할 숫자 {}", secret_number);

    loop {
        println!("answer!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("입력한 값을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            

        println!("입력한 값: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("less"),
            Ordering::Greater => println!("greater"),
            Ordering::Equal => {
                println!("equal");
                break;
            },
        }
    }
}
