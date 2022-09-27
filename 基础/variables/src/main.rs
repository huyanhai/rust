fn main() {
    let _x = 20; // 不使用的变量使用下划线开头
    let mut num = 10;
    println!("num is :{}", num);
    num = 20;
    println!("num is :{}", num);

    let (_, is_true) = (1, true); // 变量解构
    println!("is_true:{}", is_true);

    let [a, .., b, _] = [1, 2, 3, 4, 5, 6]; // ..对数组进行切片
    println!("a:{},b:{}", a, b);

    // 常量使用const,必须给定一个类型
    const MY_NUM: i32 = 20;
    println!("MY_NUM,{}", MY_NUM);

    // 变量遮蔽(shadowing),允许定义相同的变量，后面的变量会遮蔽前面的变量
    let y = 10;
    let y = 20;
    println!("x is :{}", y);
}
