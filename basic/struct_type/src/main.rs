
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    active: bool,
}

struct Reactangle {
    width: u32,
    length: u32,
}

impl Reactangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Reactangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Reactangle {
        Reactangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    println!("Hello, world!");

    // struct:自定义的数据类型
    // 定义struct：
    // 使用struct关键字，并为整个struct命名，花括号内为所有字段定义名称和类型
    
    // 实例化
    let mut user1 = User {
        name: String::from("Wstreet"),
        active: true,
        email: String::from("abc@163.com")
    };

    // 访问字段
    user1.email = String::from("asd@163.com");


    // struct实例是可变的，实例中所有字段都是可变的



    // 作为函数返回值
    let user2 = build_user(
        String::from("zxc@ccc.com"), 
        String::from("wstreet")
    );

    println!("user2: {:?}", user2);


    // struct更新语法
    // 想基于某个struct实例来创建一个新的实例的时候，可以使用该语法
    let user3 = User {
        name: String::from("user3"),
        email: user2.email,
        active: user2.active,
    };

    let user4 = User {
        name: String::from("user4"),
        ..user3
    };


    // Tuple struct
    // 类似tuple的struct叫做tuple struct
    // 整体有名，里边的元素没有名字
    // 适用：想给整个tuple起名，并让它不同于其他tuple，而且又不需要给每个元素起名

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // Unit-Like Struct
    // 没有任何字段的struct，叫做Unit-Like struct
    // 适用于需要在某个类型上实现某个trait，但是在里边有没有想要存储的数据


    // struct 数据所有权
    // struct User {
    //     name: String,
    //     email: String,
    //     active: bool,
    // }

    // let w = 30;
    // let l = 50;
    // println!("{}", area(w, l))

    // let rect = (30, 50);
    // println!("{}", area(rect))


    let rect = Reactangle {
        width: 30,
        length: 50,
    };
    // println!("{}", area(&rect))

    // struct方法
    // 第一个参数是self，表示实例

    println!("{}", rect.area());

    // 方法调用
    // Rust会自动引用或者解引用
    // p1.distance(&p2)
    // (&p1).distance(&p2),效果一样


    // 其他参数
    let rect2 = Reactangle {
        width: 10, 
        length: 10,
    };
    println!("rect can hold rect1: {}", rect.can_hold(&rect2));


    // 关联函数（静态方法？），第一个参数不是self
    // 通常用于构造器，比如String::from
    let square1 = Reactangle::square(30);

    // 过个impl块
    // 每个struct允许有多个impl块
}


fn build_user(email: String, name: String) -> User {
    User { name: name, email:email, active:true }
}


// fn area(width: u32, length: u32) -> u32 {
//     width * length
// }


// fn area(dim: (u32, u32)) -> u32 {
//     dim.0 * dim.1
// }


// fn area(rect: &Reactangle) -> u32 {
//     rect.width * rect.length
// }