// We can use a default value for the trait by passing in a method
// and calling 'impl Summary for Article {}'
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for SocialMediaPost {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Summary for Article {
    fn summarize_author(&self) -> String {
        format!("{} located at {}", self.author, self.location)
    }
}

pub struct SocialMediaPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

pub struct Article {
    pub author: String,
    pub title: String,
    pub content: String,
    pub location: String,
}
