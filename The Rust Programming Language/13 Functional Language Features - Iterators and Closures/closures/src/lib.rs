use std::thread;
use std::time::Duration;

pub fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("...ing...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_result = simulated_expensive_calculation(intensity);

    // let expensive_closure = |num| {
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("...ing...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // memoization, lazy evaluation
    let mut expensive_result = Cacher::new(|num| {
        println!("...ing...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            // simulated_expensive_calculation(intensity)
            // expensive_result
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            // simulated_expensive_calculation(intensity)
            // expensive_result
            // expensive_closure(intensity)
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                // simulated_expensive_calculation(intensity)
                // expensive_result
                // expensive_closure(intensity)
                expensive_result.value(intensity)
            );
        }
    }
}

// fn not_diff_type() {
//     let example_closure = |x| x;
//     let s = example_closure(String::from("hello"));
//     let n = example_closure(5);
// }

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, args: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(args);
                self.value = Some(v);
                v
            }
        }
    }
}

#[cfg(test)]
mod Test {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
