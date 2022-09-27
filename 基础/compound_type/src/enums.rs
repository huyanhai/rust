// 枚举
#[derive(Debug)]
enum Color {
    Red,
    White,
    Black,
}

pub fn test_enum() {
    println!("{:?},{:?},{:?}", Color::Red, Color::White, Color::Black);

    let num: Option<i32> = Some(10);

    println!("{:?}", num);
}
