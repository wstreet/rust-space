
fn main() {
    println!("Hello, world!");
    // Crate: binary 或者 libary

    // package： 包含Cargo.toml
    // - 至少包含一个Crate（library或者binary）
    // - 只能包含0-1个library crate
    // - 可以包含任意数量的binary crate

    // 惯例：
    // src/main.rs: 
    // - binary crate的crate root
    // - crate 名和package 名相同

    // src/lib.rs:
    // - package包含一个library crate
    // - library crate 的crate root
    // - crate 名与 package的名相同


    // module

}
