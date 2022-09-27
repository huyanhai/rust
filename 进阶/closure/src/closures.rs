// 闭包
// 闭包是一种匿名函数，他可以赋值给变量也可以作为参数传递给其他变量

pub fn test_closure() {
    let x = |y| y + 1;

    println!("{}", x(10));
}
