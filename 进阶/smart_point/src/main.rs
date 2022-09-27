fn main() {
    // 将数据存储到堆上
    let a = Box::new(10);
    // 数据被存到栈上
    let b = 10;

    // println!("{}", a + 1); 报错，a是一个指针
    // 对a解引用
    println!("{}", *a + 1);
}
