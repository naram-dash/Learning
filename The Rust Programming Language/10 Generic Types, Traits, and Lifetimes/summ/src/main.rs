use summ::{NewsArticle, Summary, Tweet};

fn main() {
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
