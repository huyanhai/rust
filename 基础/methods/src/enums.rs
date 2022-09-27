// 为枚举定义方法

enum Message {
    Write(String),
}

impl Message {
    fn show(&self) {}
}

pub fn test_enums() {
    let message = Message::Write(String::from("demo"));
    message.show();
}
