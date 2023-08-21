use ch10_trait::{Summary, Tweet, NewsArticle, notify};

fn main() {
    println!("Hello, world!");

    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet {}", tweet.summarize());
    }

    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        };

        println!("1 new article {}", article.summarize());
    }

    {
        // compile time error
        //  ^^^^^ the trait `Summary` is not implemented for `str`
        // notify("123");
    }

    {
        let s = 3.to_string();
    }
}
