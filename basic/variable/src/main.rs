
//  const MAX_POINTS: u32 = 100_000;
 // 数字中的_是为了增强可读性

fn main() {
    println!("Hello, world!");
    // mut关键字使变量成为可变变量
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // 常量不可以使用mut
    // 声明厂量使用const，类型必须被标注
    // 常量命名规范：全大写，多个单词使用_分开
    // const MAX_POINTS: u32 = 100_000;
    // 数字中的_是为了增强可读性




    // shadowing
    let mut x = 5;
    x = x + 7;
    let x = x + 1; // 上一行的x被shadow
    println!("The value of x is {}", x); // 新的x

    
    // shadow和mut不一样
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);



    /*
     * 标量数据类型
     * 一个标量类型代表一个单个的值
     * Rust有4个主要的标量类型
     * - 整数： i类和u类（例如i32,u32）,isize和usize由程序运行的计算机的架构决定
     * - 浮点：f32,f64，默认f64
     * - 布尔: true ,false
     * - 字符: 描述最基础的单个字符，使用单引号，占4个字节，是Unicode的标量值
     * 
     * 
     * */

     let y = 2.0;
     let w: f32 = 3.0;

     let t = true;
     let f: bool = false;

    let c1 = 'z';
    let c2: char = ' ';
    let c3 = '😊';


    /*
     * 复合数据类型:两种
     * 1、Tuple
     *  - 将多个类型的多个值放在一个类型里
     *  - 长度是固定的，一旦声明就无法修改
     * 2、数组
     *  - 数组可以将多个值放在同一个类型里
     *  - 数组中每个元素的类型必须相同
     *  - 数组的长度也是固定的
     * 
    */

    // tuple
    let tup1 = (500, 6.4, 1u8);
    
    println!("{}, {}, {}", tup1.0, tup1.1, tup1.2);

    let tup2: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup2;
    println!("{}, {}, {}", x, y, z);

    // array
    // 将数据存放在stack上而不是heap上，或者想保证有固定数量的元素，这时使用数组更好
    // 数组没有Vector灵活
    let a1 = [1, 2, 3, 4];
    // 数组类型： [类型; 长度]
    let a2 = [3;5]; // 等价于 let a2: [i32; 5] = [3,3,3,3,3]
    println!("{:?}", a2);

    /*
     * 函数
    */
    
    another_function();

    // 函数的参数:parameters, arguments,对应形参和实参
    
    
    // 函数返回值，在->后面申明返回值类型
    //返回值就是函数题中最后一个表达式的值，如果想提前返回
    // 需要使用return关键字，并指定一个值
}

// 使用fn关键字
fn another_function() {
    println!("another_function");
}

fn five() -> i32 {
    5
}


//这是单行注释

/*
 * 这是多行
 * 注释
*/