/**
 * 所有权
 */

// 悬空指针(悬垂引用) - 指针指向一个内存地址后，该地址被释放，但指针任然存在

// stack 栈 先进后出 已知占用内存大小

// heap 堆 未知大小的数据
// 1.向堆中存放数据时，操作系统先找到一块足够大的空间并标记为已使用，
// 2.返回表示该位置的指针，过程被称为在堆上分配内存
// 3.接着该指针会被存到栈中
// 4.使用的时候通过栈里的指针，找到具体位置的具体数据

// 函数参数会被压入到栈中，当函数调用完成后，会以先进后出的规则移除栈上的参数数据

// 所有权原则
// 1.Rust中每一个值都被一个变量拥有，该变量被称为值的所有者
// 2.一个值同时只能被一个变量拥有
// 3.当所有者（变量）离开作用域，这个值将被丢弃（drop）

// String为非基本类型，会发生转移
fn test_str(str: String) {
    println!("str is :{}", str);
    // 移出作用域并调用 `drop` 方法。占用的内存被释放
}

// i32为基本类型，存储在栈上，所有权不会发生转移
fn test_int(num: i32) {
    println!("num is :{}", num);
    // 移出作用域。不会有特殊操作
}

fn test_str1(str: String) -> String {
    return str;
}

pub fn test_owner() {
    let x = 5;
    let y = x;

    // 栈上的浅拷贝不会发生所有权变更，堆上的浅拷贝会发生所有权变更

    // String类型会存储在堆中
    let str1 = String::from("hello ");
    // 所有权str1转移到str2，str1失效
    let mut str2 = str1; // 堆上的浅拷贝
    let _str2_1 = str2.clone(); // 深拷贝

    // 由于rust禁止访问无效的引用，所以会报错
    // println!("{}", str1);

    str2.push_str("world");

    println!("{},{}", x, y);
    println!("{}", str2);

    // 字符串引用存放在栈上
    let str3: &str = "hello world";
    let str4 = str3; // 栈上的浅拷贝 因为栈上很快，所以深浅拷贝没有多大区别

    println!("{},{}", str3, str4);

    // 任何基本类型的组合可以copy,不需要分配内存或某种形式资源的类型也是可以copy的

    // -----------------
    let my_str = String::from("test");
    let my_num = 20;

    test_str(my_str);
    test_int(my_num);

    println!("{}", my_num);

    // println!("{}", my_str); my_str 发生的引用发生了改变

    // ------------------
    let my_str1 = String::from("my_str1");

    let _my1 = test_str1(my_str1);
}
