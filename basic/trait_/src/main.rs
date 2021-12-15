

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

    println!("1 new: {}", new_article.summarize());

    // let char_list = vec!['a', 'c', 'n', 'b'];
    let char_list = vec![String::from("_"), String::from("_")];
    let result = largest(&char_list);
}


fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for num in list.iter() {
        if num > largest { // std::cmp::PartialOrd
            largest = num;
        }
    }
    largest
}