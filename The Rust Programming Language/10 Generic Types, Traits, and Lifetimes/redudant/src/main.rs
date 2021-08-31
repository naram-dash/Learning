// 트레이트를 가져와야 실행할 수 있음
use redudant::tttrait::NewsArticle;
use redudant::tttrait::Summary;
use redudant::tttrait::Tweet;

fn main() {
    // // 리스트1
    // let number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("{}", largest);

    // // 리스트2
    // let number_list = vec![34, 50, 25, 100, 65, 102, 43, 4, 12, 3];

    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("{}", largest);

    // 리스트1
    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("{}", result);

    // 리스트2
    let number_list = vec![34, 50, 25, 100, 65, 102, 43, 4, 12, 3];
    // let result = largest(&number_list);
    // println!("{}", result);
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("study rust"),
        reply: false,
        retweet: false,
    };

    println!("새트윗 3개 , {}", tweet.summary());

    let news = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
    hockey team in the NHL.",
        ),
    };

    println!("뉴우스 3개 , {}", news.summary());
}

// fn largest(list: &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }
