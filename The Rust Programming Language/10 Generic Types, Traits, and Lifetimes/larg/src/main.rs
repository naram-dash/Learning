fn main() {
    // 리스트1
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("{}", result);

    // 리스트2
    let number_list = vec![34, 50, 25, 100, 65, 102, 43, 4, 12, 3];
    let result = largest(&number_list);
    println!("{}", result);
}

// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
