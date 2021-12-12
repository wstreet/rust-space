fn main() {
    println!("Hello, world!");
     // byte的集合
     // 能将字节解析为文本的一下方法

    // rust核心语言层面，只有一个字符串类型：字符串切片str（或者&str）
    // 切片：对存储在其他地方、UTF-8编码的字符串的引用
    // - 字符串字面值：存储在二进制文件中，也是字符串切片


    // String类型
    // - 来自标准库，而不是 核心语言
    // -可增长、可修改、可拥有所有权
    // - UTF-8编码

    // 通常说的字符串是指String和 str
    // 以String结尾的是拥有所有权的，str是借用的变体
    
    
    // 创建
    let mut s = String::new();

    let data = "init";
    let s1 = data.to_string();

    let s1 = "init".to_string();

    let s3 = String::from("init");

    // 更新
    // push_str: 把一个字符串切片附加到String
    let mut s4 = String::from("foo");
    let s5 = String::from("_");
    s4.push_str(&s5);
    s4.push_str("bar");

    println!("{}", s4);

    // push: 把单个字符附加到String
    s4.push('l');
    println!("{}", s4);

    // + （类似fn add方法）
    let s6 = s3 + &s5;
    println!("{}", s6);
    // println!("{}", s3); // s3不能使用，s3的所有权移动到了add里边
    println!("{}", s5);

    // format!
    let s7 = String::from("s7,");
    let s8 = String::from("s8,");
    let s9 = String::from("s9,");
    let formts = format!("{}-{}-{}", s7, s8, s9);
    println!("{}", formts);

    // 索引访问, 不允许
    let len = String::from("qwer").len();
    // 一个字母占用一个字节，但是其他语言每个unicode变量值不一定占几个字节

    println!("{}", len);

    // 切割,必须沿着字符边界切割
    let hello = "qwert";
    let ss = &hello[0..4];
    println!("{}", ss);

    // String并不简单！！！

}
