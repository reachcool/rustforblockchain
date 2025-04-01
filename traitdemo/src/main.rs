use traitdemo::{NewsArticle, Summary, Tweet,notify};

fn main() {
    let tweet = Tweet {
        username: String::from("X user1"),
        content: String::from(
            "X content",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    
    let news = NewsArticle {
        headline:String::from("New article1"),
        location:String::from("Beijing"),
        author:String::from("Author1"),
        content:String::from("News Content!")
    };
    notify(&news);
    let sum = returns_summarizable();
    println!("{:#?}",sum.summarize());
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}