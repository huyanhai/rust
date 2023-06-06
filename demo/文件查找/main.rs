use std::{env, fs, process};
mod lib;

use lib::{run, search, Config};

fn main() {
    // 获取命令参数 cargo run -- xxx yyy
    let args: Vec<String> = env::args().collect();
    // unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("程序异常：{}", err);
        eprintln!("程序异常：{}", err);
        process::exit(1);
    });

    let ctx = run(config).unwrap();

    let result = search("nobody", &ctx);

    println!("result is {:?}", result)
}
