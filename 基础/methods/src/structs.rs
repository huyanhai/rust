// 为结构体定义方法

struct User {
    name: String,
}

impl User {
    // 静态方法
    // 静态方法调用使用xxx::xxx
    pub fn new(name: String) -> User {
        User { name }
    }

    // 实例方法
    // 实例方法调用使用xxx.xxx
    // 第一个参数是&self
    fn show(&self) -> String {
        return self.name.clone();
    }

    fn set_name(&mut self, name: &str) {
        self.name.push_str(name);
    }
}

pub fn test_structs() {
    let mut user = User::new(String::from("张三"));

    user.set_name("demo");

    println!("{}", user.show());
}
