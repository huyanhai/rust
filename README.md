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
