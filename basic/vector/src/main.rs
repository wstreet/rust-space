use std::{ptr::NonNull, vec};

fn main() {
    println!("Hello, world!");

    // Vector
    // Vec<T>
    // 标准库提供
    // 可存储多个值
    // 只能存储相同类型的值
    // 值在内存中连续存放


    // 创建
    // 1.new创建，vec 是空的
    let v: Vec<i32> = Vec::new();

    // 2. 使用初始值创建， vec! 宏
    let v1 = vec![1, 2, 3];

    // 添加元素
    let mut v2= Vec::new();
    v2.push(1);

    // 删除
    // 离开作用域会被删除


    // 读取
    let third:&i32 = &v1[2];

    match v1.get(2) {
        Some(third) => println!("{}", third),
        None => println!("None")
    }


    // 遍历
    let mut vv = vec![100, 32, 78];
    for i in &mut vv {
        // println!("{}", i)
        *i += 50
    }
    println!("{:#?}", vv);


    // 通过枚举的变体，可以在vector中存放不同类型数据

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}
