fn main() {
    

    // 所有权解决的问题：
    // 1、跟踪代码的哪部分正在使用heap的哪些数据
    // 2、最小化heap上的重复数据
    // 3、清理heap上的未使用的数据以避免空间不足

    // 管理heap数据是所有权存在的原因

    /*
     * 所有权的规则
     * - 每一个值都有一个变量，这个变量是该值的所有者
     * - 每个值同时只能有一个所有者
     * - 当所有者超出作用域时，将该值删除
     * 
    */

    let s = "hello";

    // String类型
    // 在heap上分配空间

    let mut s1  = String::from("Hello");
    s1.push_str(", World");

    println!("{}", s1);

    // 变量和数据交互的方式：
    // （1）、move
    let x=  5;
    let y = x;

    // 一个String类型由3部分组成
    // 1、ptr:存放内容的指针
    // 2、len:长度
    // 3、capacity: 容量
    // ptr， len， capacity存放在stack上，存放字符串内容的部分放在heap上
    let s2 = String::from("hello");
    let s3 = s2; // s2赋值给s3时，String的数据被复制了一份，s2和s3的ptr指向同一块内存

    // 当作用域结束的时候，s2和s3会被释放相同的内存，引起double free bug
    // 为了保证安全，rust让s2失效了(所有权move)
    // println!("s2 is {}", s2)


    // （2）clone，针对heap上的数据
    let s4 = s3.clone(); //clone 消耗资源
    // stack上的直接复制


    // （3）copy
    // 任何简单标量的组合类型都是可以实现copy的
    // 任何需要分配内存或者某种资源的都不是copy的


    // ----- 所有权和函数-------
    let s5 = String::from("Hello");

    take_ownership(s5);

    let xx = 5;

    make_copy(xx);

    println!("xx: {}", xx);
    
    
    // 返回值与作用域


    let s6 = give_some_ownership();
    let s7 = String::from("hello s7");
    let s8 = take_and_give_back(s7); 

    // print!("s6: {}, s7: {}, s8: {}", s6, s7, s8)


} // s作用域到此结束


fn take_ownership(some_string: String) {
    println!("{}", some_string)
}

fn make_copy(some_number: i32) {
    println!("{}", some_number)
}

fn give_some_ownership() -> String {
    let s =  String::from("Hello give");
    s
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}