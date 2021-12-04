fn main() {
    // rust中两种不持有所有权的数据类型
    // 1、引用
    // 2、切片

    let mut s = String::from("Hello world");
    let word_index = first_word(&s[..]);

    // s.clear();

    println!("{}", word_index);


    // 字符串切片,类型表示：&str
    // 形式：[开始索引..结束索引]，包含开始，不包含结束
    let s1 = String::from("Hello world");

    let hello = &s1[0..5];
    let world = &s1[6..11];
    let hello1 = &s1[..5];
    let world1 = &s1[6..];
    let world2 = &s1[6..s1.len()];

    println!("{}, {}", hello, world);
    println!("{}, {}, {}", hello1, world1, world2);

    let whole = &s1[..];
    println!("{}", whole);


    // 字符串字面值是切片
    // 直接存储在二进制程序中的
    let ss = "Hello World!";

     // 将切片作为参数传递
     let word_index = first_word(ss);


     // 其他类型切片
     let a = [1, 2, 3];
     let slice = &a[0..3];
     println!("arr slice is {:?}", slice)

}


// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return  i;
//         }
//     }
//     s.len()
// }

// 重写
// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();
//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }
//     &s[..]
// }

// 重写
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}