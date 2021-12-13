
use std::{fs::File};
use std::io::{self, ErrorKind, Read};

fn main() {
    println!("Hello, world!");

    // 分类
    // 可恢复
    // 不可恢复（bug）

    // rust没有异常机制
    // - 可恢复错误： Result<T,E>
    // - 不可恢复：panic!宏，程序立即终止执行


    // panic!("crash and burn")

    let v = vec![1, 2, 3];
    // v[99];


    // Result枚举
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // Err(error) => {
        //     panic!("Error opening file {:?}", error);
        // }

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) =>   panic!("Error creating file {:?}", e),
            },
            other_error => panic!("Error opening file {:?}", other_error),
        }
    };

    // match有用，但是原始

    // unwrap：macth表达式的快捷方法
    
    let f = File::open("hello.txt").unwrap();
    // 相当于以下代码
    // let f = File::open("hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Error opening file {:?}", error);
    //     }
    // };

    // expect
    let f = File::open("hello.txt").expect("无法打开文件");


    // 传播错误
    let result = read_username_from_file();


    // 什么时候应该使用panic
    // 在定义一个可能失败的函数时，优先考虑返回Result
    // 否则使用panic


}


// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     // 等价
//     // let mut f = match File::open("hello.txt") {
//     //     Ok(file) => file,
//     //     Err(error) => return Err(error) // return Err(error)是指将 Err(error)作为函数返回值
//     // };

    
//     let mut s = String::new();

//     f.read_to_string(&mut s)?;
//     Ok(s)
// }


// 简化
fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
 
}

// ？运算符只能用于返回Result的函数
