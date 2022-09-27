// 结构体

// 定义结构体
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    address: String,
}

fn create_user(name: String, age: i32, address: String) -> User {
    User { name, address, age }
}

// 元祖结构体
#[derive(Debug)]
struct Color(i32, i32);

fn create_tuple(one: i32, two: i32) -> Color {
    return Color(one, two);
}

pub fn test_struct() {
    let user1 = create_user(String::from("张三"), 12, String::from("重庆"));

    let user2 = User {
        name: String::from("李四"),
        ..user1
    };

    println!("{},{}", user1.name, user1.age);
    println!("{},{},{}", user2.name, user2.age, user2.address);
    println!("{:?}", user2);

    // println!("{}", user1.address); user1上面的address使用权发生转移

    let tuple = create_tuple(1, 2);
    println!("tuple is :{:#?}", tuple);
    // 使用dbg宏打印信息
    dbg!(tuple);
}
