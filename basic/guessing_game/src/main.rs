use std::io; // prulude
use rand::Rng; // trait
use std::cmp::Ordering;


fn main() {
    println!("猜数游戏🎮");

    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("神秘数字是{}", secret_num);
    println!("请输入一个数！");

    // mut 可变的变量
    let mut guess = String::new();

    // 传递可变引用
    io::stdin().read_line(&mut guess).expect("无法读取行");

    // 新的guess shadow 旧guess，后面的guess都是指新的guess
    let guess:i32 = guess.trim().parse().expect("Please type a number!");


    // read_line 返回枚举io::Result Ok, Err
    println!("你猜的数是：{}", guess);
    // {} 占位符，被guess替换

    match guess.cmp(&secret_num) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("too big"),
        Ordering::Equal => println!("You win!")
    }
}
