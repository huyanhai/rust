use std::ops::Add;

#[derive(std::fmt::Debug)]
struct Num1(u32, u32);

#[derive(std::fmt::Debug)]
struct Num2(u32);

// 重载加法运算符
impl Add<Num2> for Num1 {
    type Output = Num1;
    fn add(self, other: Num2) -> Num1 {
        return Num1(self.0 + other.0, self.1);
    }
}

//
trait Person {
    fn show_name() -> String;
}

struct Child;

impl Child {
    fn show_name() -> String {
        return String::from("baby");
    }
}

impl Person for Child {
    fn show_name() -> String {
        return String::from("child");
    }
}

pub fn test_adv() {
    let num = Num1(1, 2);
    let num2 = Num2(2);

    println!("{:?}", num.add(num2));

    println!("show_name :{}", Child::show_name());
    println!("show_name :{}", <Child as Person>::show_name());
}
