use aggregator::{Article, SocialMediaPost, Summary};

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize_author())
}

fn returns_summerizable() -> impl Summary {
    SocialMediaPost {
        username: String::from("Slydog969"),
        content: String::from("This is my first post!"),
        reply: false,
        repost: false,
    }
}

fn main() {
    let my_article = Article {
        author: String::from("Cameron Guilbeau"),
        title: String::from("White Feather"),
        content: String::from("The following is an article about living in the far north"),
        location: String::from("Tromso, Norway"),
    };

    let my_social_post = returns_summerizable();

    println!("{}", my_article.summarize());
    println!("{}", my_social_post.summarize());
    notify(&my_article);
    notify(&my_social_post);
}
