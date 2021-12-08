
// module
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add_to_waitlist")
    }

    fn some_function() {
        
    }

    fn seat_at_table() {
        
    }
  }

  mod serving{
    fn take_order() {
        
    }

    fn take_payment() {
        
    }
  }
}

// use 引入模块,私有模块不会引入
// use front_of_house::hosting; // 相对路径
use crate::front_of_house::hosting; // 绝对路径
// 相当于mod hosting {}
// 针对函数，一般引入上一级模块
// struct，enum一般指定完整路径，直接引入本身
// 同名模块只引入到上级就行，也可以as指定别名

use std::fmt::Result;
use std::io::Result as IoResult;
// pub use std::io::Result as IoResult; // 外部可以访问到

pub fn eat_at_restaurant() {
  // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    // use path
    hosting::add_to_waitlist()
    
}


// 

fn serve_order() {
        
}

mod back_of_house {
  fn fix_incorrect_order() {
    // super
      super::serve_order()
  }

  fn cook_order() {
      
  }

  // pub 放在struct前面
  // struct是公共的
  // struct的字段是私有的
  pub struct Breakfast {
    // 字段需要单独加 pub 变成共有的
  }

   pub enum Appetizer {
       Soup,
       Salad, // 枚举成员不需要加pub
   }
}

