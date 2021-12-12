use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    // 创建
    // new insert
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);

    // 存储在heap中
    // k是同一种类型，v是同一种类型

    // collect方法
    // 在元素类型为Tuple的Vector上使用collect方法。可以组建一个HashMap
    // - 要求Tuple有两个值：一个为k，一个为v
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let init_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(init_scores.iter()).collect();


    // 所有权
    // 对于实现了copy trait的类型（i32等），值会被复制到HashMap中
    // 对于拥有所有权的值（String等），值会被移动，所有权会转移给HashMap

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    // map.insert(field_name, field_value);

    // println!("{}: {}", field_name, field_value) // field_name, field_value实效


    map.insert(&field_name, &field_value); // 将引用插入到hashmap中
    println!("{}: {}", field_name, field_value);


    // 访问
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("{}", s),
        None => println!("None")
    }

    for (k, v) in scores {
        println!("{}: {}", k, v)
    }


    // 更新
    // - 替换
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);

    // - 不存在才插入 Entry 的 or_insert

    let mut scores = HashMap::new();
    scores.entry(String::from("Blue")).or_insert(50);

    // 基于现有值更新
    let text = "hello world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert((0));
        *count += 1;
    }
    println!("{:#?}", map);

}
