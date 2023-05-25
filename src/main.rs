use std::{env, fs};

fn main() {
    // 获取命令参数 cargo run -- xxx yyy
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let name = &args[2];

    println!("path:{},", path);

    let ctx: String = fs::read_to_string(path).expect("file is not found");

    println!("{}", ctx);
}
