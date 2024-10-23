### 模块
```rs
// 定义并导出模块
// ./test.rs
pub fn test() {}


// ./main.rs

// 导入模块
mod test;

// 使用模块的方法
test::test();


// 目录下的模块
//./test/mod.rs
// 导入并导出模块
pub mod test;


//./test/test.rs
pub fn test() {}
```

### 所有权转移

```rs
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;
    println!("s2: {s2}");
    // println!("s1: {s1}");
}
```

- 将 s1 赋值给 s2 后，s1 的所有权发生了转移，以后 s1 将不可访问
- 当离开 s2 的作用域后，s1 绑定的变量会被释放

### 借用

```rs
fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    // let p4: Point = add1(p1, p2); 这里会发生所有权转移
    println!("{p1:?} + {p2:?} = {p3:?}");
}
```

### struct(结构体)

struct 理解为 java 重的类，类里面有属性和方法

```rs
// 定义结构体
struct Library {
    book: Vec<Book>,
}

// 为结构体实现方法
impl Library {
    fn new(book: Book) -> Library {
        Library { book: vec![book] }
    }

    fn add(mut self, book: Book) {
        self.book.push(book);
    }

    fn len(self) -> usize {
        self.book.len()
    }
}
```

### trait(接口)

接口类似 java 重的抽象类，只定义了基本的类型/方法，但是没有具体的实现

```rs
// 定义结构体
struct Library {
    book: Vec<Book>,
}
// 定义接口
trait Page {
    fn show_page(book: Book) -> usize;
}
// 用接口实现结构体
impl Page for Library {
    fn show_page(book: Book) -> usize {
        book.title.len()
    }
}
```

### enum(枚举)

```rs
enum WebEvent {
    PageLoad,
    KeyPress(char),
    Click { x: u16, y: u16 },
}

let web1 = WebEvent::PageLoad;
let web2 = WebEvent::KeyPress('c');
let web3 = WebEvent::Click { x: 1, y: 1 };

fn test(web: WebEvent) {
    match web {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::KeyPress(x) => println!("x is {x:}"),
        WebEvent::Click { x, y } => println!("x is {x:},y is {y:}"),
    }
}

test(web1);
test(web2);
test(web3);
```

### 解构

```rs
let arr = [1, 2, 3];

match &arr {
    // [x, y, z] => println!("x {},y {},z {}", x, y, z),
    // [x, ..] => println!("x {}", x),
    _ => print!("default"),
}
```

### Result 和 Option

```rs
fn find_book(self, str: &str) -> Result<Book, &str> {
    for bk in self.books {
        if bk.title == str {
            return Ok(bk);
        }
    }
    return Err("not found");
}

fn test_op(x: u8) -> Option<String> {
    if x > 10 {
        return Some(String::from(""));
    } else {
        return None;
    }
}

let result: Option<String> = tst_op(10);
let result1 = library.find_book("语文");

match result {
    Some(x) => println!("{}", x),
    _ => println!("default"),
}

match result1 {
    Ok(book) => println!("the book is {},year is {}", book.title, book.year),
    Err(str) => println!("{str:}"),
}
```

### HashMap(类似 json)

```rs
let mut obj = HashMap::new();

obj.insert("name", "语文");
obj.insert("name", "数学");
obj.insert("year", "1991");
// 同名的会被后面的覆盖
// {"year": "1991", "name": "数学"}

let user_list: Vec<(&str, u16)> = vec![("name1", 12)];

let mut user_map: HashMap<&str, u16> = user_list.into_iter().collect();

println!("user_list: {:?}", user_map);
// user_list: {"name1": 12}

println!("user_list: {:?}", user_map.get("name1"));
// user_list: Some(12)

println!("user_list: {:?}", user_map["name1"]);
// user_list: 12

// 如果存在则返回name2对应的值，否则插入name2->20,并返回插入后的值
let i = user_map.entry("name2").or_insert(20);
```

### 静态数组和动态数组
```rs
// 生成100个1的数组
// 静态数组，长度固定
let list: [i32; 100] = [1; 100];

// 动态数组
let mut list1: Vec<i32> = vec![1, 2, 3];

list1.push(4);

// 在遍历的时候需要修改元素的时候使用
list.into_iter
let vec = vec![1, 2, 3];

// 使用 iter 遍历
for &value in vec.iter() {
    println!("{}", value); // 打印出 1, 2, 3
}

// vec 仍然可以使用
println!("{:?}", vec); // 打印出 [1, 2, 3]
}

// 在遍历时不需要修改元素的时候使用
list.iter
let vec = vec![1, 2, 3];

// 使用 into_iter 遍历
for value in vec.into_iter() {
    println!("{}", value); // 打印出 1, 2, 3
}

// vec 现在已经不可用
// println!("{:?}", vec); // 错误：vec 已被消费
```

### Box<T>

```rs
let box = Box::new(5);
// b指向定义在堆上的Box(5)
```

### Result<T,E>

unwrap_or_else 是定义在 Result<T,E> 上的常用方法，如果 Result 是 Ok，那该方法就类似 unwrap：返回 Ok 内部的值；如果是 Err，就调用闭包中的自定义代码对错误进行进一步处理

```rs
fn build(args: &[String]) -> Result<Self, &str> {
    if args.len() < 3 {
        return Err("not enough arguments");
    }
    let query = args[1].clone();
    let file_path = args[2].clone();

    Ok(Config { query, file_path })
}

let config = Config::build(&args).unwrap_or_else(|err| {
    println!("程序异常：{}", err);
    process::exit(1);
});
```

### 闭包

```rs
|x: u8| {
    let y = 10;
    x + y
}

FnOnce // 该闭包只运行一次

fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    println!("{}", func(4));
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()})
}

FnMut // 可变
```

### 迭代器

```rs
struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 10 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
```

### 静态字符串和动态字符串
```rs
let str = "hello world";

let string = String::from("hello world");

// 将静态字符串转为动态字符串
let mut string1 = str.to_string();

// 将动态字符串转为静态字符串
let str1 = string.as_str();

string1.push_str("info");

let string2 = String::from("hello,世界");

string1.push('!');

// 汉字占3个字节
println!("{}", string1);
```

### 项目结构
- module：用于组织和封装代码的单元。它可以包含函数、结构体、枚举、常量、Trait等，也可以包含其他模块。通过使用mod关键字可以在 Rust 中创建模块，并且模块可以嵌套形成模块树。
- crate：是 Rust 中的编译单元，可以是库 crate 或二进制 crate（它们的区别参见FAQ）。一个 crate 可以包含一个或多个模块。
- package：是一个包含一个或多个相关 crate 的项目工程。它由一个 Cargo.toml文件定义，该文件包含有关项目的元信息、依赖关系以及其他配置选项。一个 package 可能包含一个主 crate（通常是可执行文件）和零个或多个库 crate（通常是依赖 crate）。