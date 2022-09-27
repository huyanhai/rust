fn add(x: i32, y: i32) -> i32 {
    let a = x + y;
    return a;
}

// () 表示单元类型
// fn example() -> () {}

// 语句会执行一些操作但不会有返回值
// 表达式总有返回值，表达式不能包含分号

fn main() {
    // i-有符号 表示可以是正数也可以试负数
    // u-无符号 表示只能是正数
    // -(2^7) ~ 2^7 - 1 有符号可表示的数据长度 -128 ~ 127
    let _x: i8 = -128;
    let _y: u32 = 10;

    // isize usize - 具体位数取决于系统架构 32位为 i32/u32 64位为i64/u64
    let _z: isize = 12;

    // 整形溢出,debug模式会提示异常，编译时会自动按照补码循环溢出处理，大于该类型会变成该类型的最小值
    // let _x1: u8 = 256; -> 0

    // 存在精度问题
    // assert!(0.1 + 0.2 == 0.3);

    println!("{}", (0.1_f64 + 0.2 - 0.3).abs());

    // 字符类型 一个字符4个字节
    let char1 = '国';
    println!("char1 size is:{}", std::mem::size_of_val(&char1));

    // 表达式
    let _x1 = {
        let nums = 10;
        nums + 1
    };

    println!("add:{}", add(10, 20))
}
