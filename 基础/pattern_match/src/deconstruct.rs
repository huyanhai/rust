// 解构

struct User {
    name: String,
    age: i32,
}

// 解构结构体
fn test_struct() {
    let user = User {
        name: String::from("张三"),
        age: 20,
    };

    let User { name: n1, age: a1 } = user;

    println!("n1={},a1={}", n1, a1);
}

// match匹配元祖
fn test_tpl() {
    let tpl = (1, 2, 3, 4, 5, 6);

    // _不会绑定变量，
    // _a会绑定变量，但是告诉编译器该变量不会被使用

    match tpl {
        // ..可忽略剩余值
        (one, _, two, _, three, ..) => {
            println!("one is :{},two is :{},three is :{}", one, two, three)
        }
        _ => println!(""),
    }
}

// @绑定
fn at_bind() {
    let num: i32 = 20;

    match num {
        // 匹配num的值并绑定给x
        x @ 19..=21 => println!("{} matched", x),
        _ => println!("no matched"),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// @绑定
fn test() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}

pub fn test_dec() {
    test_struct();

    test_tpl();

    at_bind();

    test();
}
