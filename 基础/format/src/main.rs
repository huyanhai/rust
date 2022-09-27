use std::fmt;

struct Person {
    name: String,
}

// 为Person实现Display
impl fmt::Display for Person {
    fn fmt(&self, word: &mut fmt::Formatter) -> fmt::Result {
        write!(word, "这是一个自定义的format,name is {}", self.name)
    }
}

fn main() {
    let p = Person {
        name: String::from("张三"),
    };
    println!("{}", p);
}
