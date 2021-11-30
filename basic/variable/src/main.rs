
//  const MAX_POINTS: u32 = 100_000;
 // 数字中的_是为了增强可读性

fn main() {
    println!("Hello, world!");
    // mut关键字使变量成为可变变量
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // 常量不可以使用mut
    // 声明厂量使用const，类型必须被标注
    // 常量命名规范：全大写，多个单词使用_分开
    // const MAX_POINTS: u32 = 100_000;
    // 数字中的_是为了增强可读性




    // shadowing
    let mut x = 5;
    x = x + 7;
    let x = x + 1; // 上一行的x被shadow
    println!("The value of x is {}", x); // 新的x

    
    // shadow和mut不一样
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces)


}
