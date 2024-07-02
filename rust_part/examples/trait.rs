pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("qwe"),
        content: String::from("asd"),
        reply: true,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Rust Programming Language"),
        location: String::from("Everywhere"),
        author: String::from("The Rustaceans"),
        content: String::from("Rust is a systems programming language..."),
    };

    notify(&tweet);
    notify(&article);
}
