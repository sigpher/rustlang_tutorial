use ch10::{longest, notify, NewsArticle, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebook"),
        content: String::from("of course, as you probably alreay know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins with the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL",
        ),
    };
    println!("New article available! {}", article.summarize());

    notify(&article);

    let x = "choi";
    let y = "sigpher";

    let longest = longest(x, y);
    println!("{longest}");
}
