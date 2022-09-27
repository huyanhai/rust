// 泛型
// std::ops::Add<Output = T> 约束T可以执行加法
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    return a + b;
}

struct Point<T> {
    x: T,
    y: T,
}

// 泛型接口定义方法
impl<T> Point<T> {
    fn get_x(&self) -> &T {
        return &self.x;
    }
}

pub fn test_gen() {
    add(1, 2);
}
