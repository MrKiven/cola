pub trait Summarizable {
    fn summary(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct Default {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for Default {}


fn main() {
    let tweet = Tweet {
        username: String::from("MrKiven"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let default = Default {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
                              hockey team in the NHL."),
    };

    println!("One new tweet: {}", tweet.summary());
    println!("New article available!: {}", default.summary());
}
