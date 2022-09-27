use std::fs::File;
use std::io;
use std::io::Read;

fn open_file() {
    let f = File::open("main.rs");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("文件读取错误,{:?}", error),
    };

    println!("文件内容是:{:?}", f);
}

fn open_file1() -> Result<String, io::Error> {
    let mut s = String::new();

    // 可选链 ?.
    // 如果open的结果为Ok()则可以继续调用下面的方法，否则返回Err()
    // 最后的问号如果为Ok()则返回结果，否则返回Err()
    File::open("/Users/yhh/Documents/study/rust/基础/panic/src/main.rs")?.read_to_string(&mut s)?;

    return Ok(s);
}

fn main() {
    // open_file();

    let s = open_file1();

    println!("{:?}", s);
}
