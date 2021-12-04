fn main() {
    println!("Hello, world!");

    let mut s1 = String::from("Hello");
    {
        let s2 = &s1;
    }

    let len = calculate_len(&mut s1); // 传递引用，而不是将所有权给出去

    println!("The length of {} is {}", s1, len);


    /*
     * &表示引用：允许引用某些值而不取得其所有权
     * 
     * 以引用作为函数参数的行为，叫做借用
     *  
    */


    // 可变引用的限制：
    // 特定作用域块中，对某一块数据，只能有一个可变的引用，在编译的时候防止数据竞争

    // 不可以同时拥有一个可变引用和一个不变的引用

    // 悬空引用 Dangling References
    // 一个指针引用了内存中的某个地址，而这块内存可能已经释放并分配给了其他人使用了


}


fn calculate_len(s: &mut String) -> usize {
    s.push_str(", World");
    s.len()
}

// Dangling References
fn dangle() -> &String {
    let s = String::from("hello dangle");
    &s
}