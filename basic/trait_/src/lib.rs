
pub trait Summary {
  fn summarize(&self) -> String;
  fn summarize1(&self) -> String { // 默认实现
    format!("(Read more from {}...)", self.summarize())
  }
}

pub struct NewArticle {
  pub headline: String,
  pub location: String,
  pub author: String,
  pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline,self.author,self.location)
    }

    fn summarize1(&self) -> String { // 可以重写
      String::from("(Read more rewrite...)")
    }
}