// 特征对象
use std::ops::Add;

trait Animal {
    fn say();
}

struct Dog<T: Animal> {
    name: String,
    animal: T,
}

impl<T> Dog<T>
where
    T: Animal,
{
    fn say() {}
}

// Self 与 self
// self 指代当前实例
// Self 指代特征或类型别名 例如：Animal

pub fn test_obj() {}
