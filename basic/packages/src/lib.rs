
// module
mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {
        println!("add_to_waitlist")
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

pub fn eat_at_restaurant() {
  // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist()
    
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

