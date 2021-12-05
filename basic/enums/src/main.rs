


// 枚举
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrKind1 {
    V4(u8, u8, u8, u8),
    V6(String),
}


// 枚举定义方法
impl IpAddrKind1 {
    fn call(&self) {
        
    }
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alasks
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);
    route(IpAddrKind::V6);


    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };


     // 将数据添加到枚举的变体中
    let home1 = IpAddrKind1::V4(127, 0, 0, 1);


    // Option枚举
    // 类似于Null概念的枚举Option<T>, Option<T>和T不是同一个中类型

   let some_number = Some(5);
   let some_string = Some("A String");
   let absent_number: Option<i32> = None;


  
 // match： 模式匹配
   value_in_cents(Coin::Quarter(UsState::Alabama));

   // 匹配Option<T>
   let five = Some(5);
   let six = plus_one(five);
   let none = plus_one(None);
   println!("{:?}", six);


   // 必须穷举所有可能，下划线通配符_代替

   let v = 0u8;
   match v {
       1 => println!("one"),
       _ => (),
   }

   // if let
   // 处理只关心一种匹配而忽略其他匹配的情况

   let vv = Some(0u8);
   match vv {
       Some(3) => println!("three"),
       _ => println!("others"),
   }
    // 等效
   if let Some(3) = vv {
    println!("three")
   } else {
    println!("others")
   }



    
}

fn route(ip_kind: IpAddrKind) {
    
}

fn  value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 10,
        Coin::Dime => 5,
        Coin::Quarter(state) =>  {
            println!("State quarter from {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
}