fn main() {
    let num = 3;
    if num < 5 {
        // if后面条件必须是bool类型的
        println!("condition was true")
    } else {
        println!("condition was false")
    }

    // if是个表达式，可以 在let语句中使用if
    // if else 块中返回的类型必须相同
    let number = if num < 5 { 5 } else { 6 };
    println!("number is {}", number);


    // 循环
    // loop, while, for
    // 1. loop反复执行，直到喊停
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter *2;
        }
    };

    println!("result is {}", result);


    // 2. while

    let mut num2 = 3;
    while num2 != 0 {
        println!("{}", num2);
        num2 -= 1
    }

    println!("LIFTOFF!!");


    // 3. for 遍历集合
    let arr = [3;5];
    for item in arr.iter() {
        println!("{}", item)
    }

    for num in (1..4).rev() { // (1..4) range ,包含1，不包含4
        println!("{}", num)
    }
    println!("LIFTOFF!!");

}
