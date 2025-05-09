use m_traits::{Summary, Tweet, NewsArticle};

fn main() {
    let tweet = Tweet {
        username: String::from("Aswatt"),
        content: String::from(
            "of course, its trump"
        ),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from(
            "Penguins win the Stanley Cup Championship!"
        ),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        contents: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
            ),
    };

    println!("New tweet! {}", tweet.summarize());
    println!("New Notification! {}", article.summarize());
}
