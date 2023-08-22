#[test]
fn iterator_demonstration() {
    let v1 = vec![1,2,3];

    // iterator need to be mutable
    // let v1_iter = v1.iter();
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // immutable로 선언했지만 
    // sum fn에서 ownership을 가져갔고 (next를 써야했기 때문에 라고 추정)
    // 그래서 next를 기반으로한 consuming adapter 일 수 있었고 (immut이면 next 못돌림)
    // ownership을 가져갔으므로 이제 밑에서 못쓴다
    let total = v1_iter.sum::<i32>();


    // compile time error
    // no ownership, moved on upper statement
    // println!("{:#?}", v1_iter);

    assert_eq!(total, 6);
}


// struct KillMe {
//     a: String,
//     b: String,
// }
//
// no compile
// fn igotthat(killme: mut KillMe) {
//     killme.a = String::from("뮻");
// }

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // into_iter
    // Creates a consuming iterator, that is, one that moves each value out of the vector (from start to end). 
    // The vector cannot be used after calling this.
    //
    // iter은 &self
    // into_iter은 self => 소유권을 가져간다
    // shoes.iter().filter(|s| s.size == shoe_size).collect()
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}