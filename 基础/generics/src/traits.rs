// 特征 - 类似抽象类
// 抽象类
trait Person {
    // 抽象方法
    fn say(&self) -> String;
}

struct User {
    name: String,
}

// 给User实现Person接口方法
impl Person for User {
    fn say(&self) -> String {
        return self.name.clone();
    }
}

//---------------
// trait默认实现
trait Animal {
    // 默认实现
    fn say(&self) {
        println!("Animal say hello")
    }
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

// 采用默认实现
impl Animal for Dog {}

// 重载
impl Animal for Cat {
    fn say(&self) {
        println!("Cat say hello");
    }
}

// 使用实现了接口的方法做参数
// fn say_more(item: &impl Animal) {
//     println!("{:?}", item.say());
// }
// 等价
fn say_more<T: Animal>(item: &T) {
    println!("{:?}", item.say());
}

// 接口的多重约束
// 要求参数实现Animal特征和Person特征
// fn say_once(item: &(impl Animal + Person)) {}
// 等价
fn say_once<T: Animal + Person>(item: &T) {}

// Where约束
// fn say_onces<T: Animal + Person, U: Animal + Person>(item1: &T, item2: &U) {}
// 等价
// 使用where约束T和U的类型
fn say_onces<T, U>(item1: &T, item2: &U)
where
    T: Animal + Person,
    U: Animal + Person,
{
}

pub fn test_traits() {
    let user = User {
        name: String::from("张三"),
    };

    println!("{}", user.say());

    let dog = Dog {
        name: String::from('狗'),
    };

    let cat = Cat {
        name: String::from("猫"),
    };

    dog.say();
    cat.say();

    say_more(&cat);
}
