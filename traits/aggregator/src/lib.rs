pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Article {
    pub author: String,
    pub title: String,
    pub content: String,
    pub location: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.title, self.author, self.location)
    }
}

pub struct SocialMediaPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialMediaPost {
    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}
