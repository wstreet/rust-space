use std::fmt::{Display, Debug};


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

pub struct Tweet {
  pub usename: String,

}

impl Summary for Tweet {
  fn summarize(&self) -> String {
    format!("{}", self.usename)
  }
}

// impl trait
pub fn notify1(item: impl Summary + Display) {
  println!("Breaking news! {}", item.summarize())
}


// trait bound
pub fn notify<T: Summary + Display>(item: T, item2: T) {
  println!("Breaking news! {}", item.summarize())
}

// where子句
pub fn notify2<T, U>(item: T, item2: U) 
where
 T: Summary + Display,
 U: Clone + Debug,
{
  println!("Breaking news! {}", item.summarize())
}


pub fn notify3(item: impl Summary + Display) -> impl Summary {
  NewArticle{
    headline: String::from("..."),
    location: String::from("..."),
    author: String::from("..."),
    content: String::from("..."),
  }
  // 只能返回确定的同一种类型，返回可能不听类型的代码会报错
}
