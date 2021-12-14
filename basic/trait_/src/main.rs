
use trait_::{NewArticle, Summary};




fn main() {
    println!("Hello, world!");

    // 与interface相似

    let new_article = NewArticle{
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    println!("1 new: {}", new_article.summarize())

}
