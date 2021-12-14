
struct  Point<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}

impl<T> Point<T> {// 表示在类型T上实现x方法
    fn x(&self)  -> &T {
        &self.x
    }
}

impl Point<i32> {// 表示在i32上实现x方法
    fn x1(&self)  -> &i32 {
        &self.x
    }
}

fn main() {
    println!("Hello, world!");

    // 找最大值
    let num_list = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&num_list));
    
    let point = Point{x: 5, y: 9};
}


fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}