fn main() {
    println!("Hello, world!");

    {
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("the max is configured to be {}", max),
            _ => (),
        }
    }
    {
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("the max is configured to be {}", max);
        }
    }

    {
        let coin = Coin::Penny;
        let mut count = 0;
        match coin {
            Coin::Quater(state) =>  println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
    }
    {
        let coin = Coin::Quater(UsState::Alabama);
        let mut count = 0;

        if let Coin::Quater(state) = coin {
            println!("State quarter from {:?}!", &state);
        } else {
            count += 1;
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quater(UsState),
}