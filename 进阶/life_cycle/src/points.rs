// 悬垂指针
pub fn test_point() {
    // let x;
    {
        let y = 10;
        // x = &y;
        // y离开作用域，被释放,x任然指向y
    }

    // println!("x is {}", x);
}

// 生命周期标注 '
// 表示变量的存活时间
// 返回值'a表示 x,y中存活时间短的一个
// 一般引用类型才需要用到'生命周期
// 参数叫输入生命周期
// 返回值叫输出生命周期
/*
表示a的生命周期必须比b的长
where
    'a: 'b,
*/
pub fn largest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str
where
    'a: 'b,
{
    if x.len() < y.len() {
        y
    } else {
        x
    }
}

// 结构体生命周期
#[derive(Debug)]
pub struct Point<'a> {
    pub x: &'a i32,
    pub name: String,
}

// 返回的引用类型如果1或者2，则可以省略生命周期标注
// 1.从参数获取
// 2.从函数体内部新创建的变量获取
pub fn test_arr(s: &str) -> &str {
    &s
}

struct Animal<T> {
    x: T,
    y: T,
}

impl<T> Animal<T> {
    fn show(&self) -> &T {
        &self.x
    }
}

fn test() {
    let a = Animal { x: 10, y: 10 };

    println!("{}", a.show());
}
