use closures::*;

fn main() {
    // println!("Hello, world!");
    // // simulated_expensive_calculation(4);

    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // fn hi() -> i32 {
    //     3
    // }

    // generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // println!("변수 x를 사용할 수 없습니당: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
