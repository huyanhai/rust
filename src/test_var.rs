use std::{borrow::BorrowMut, env, fmt::Result, fs::File, io::Read, string};

#[derive(Debug)]
struct Foo {
    x: u32,
}

fn test(foo: &mut Foo) -> &mut Foo {
    foo.x += 2;
    foo
}

pub fn test_var() {
    let mut str = Foo { x: 1 };

    let mut str1 = test(&mut str);

    let mut str2 = Foo { x: 2 };

    // 这两行代码等价
    // let str3 = str2.borrow_mut();
    let str3 = &mut str2;

    println!("{:?}", str3);

    // let str3 = str2.borrow_mut();

    // 由于str的所有权被移动，所以下面这行会报错
    // println!("{:?}", str);

    println!("{:?}", str1);
    // Foo { x: 3 }
    println!("{:?}", str);
    // Foo { x: 3 }

    println!("{:?}", str2);
}

pub fn test2() {
    let mut foo = Foo { x: 3 };
    let mut foo1 = &mut foo;

    foo1.x += 1;

    println!("{:?}", foo1);
    // 当下面没有使用foo1时，foo1会自动释放，所有权回到foo
    println!("{:?}", foo);

    foo.x += 1;

    println!("{:?}", foo);
}

pub fn test_arr() {
    let names = [String::from("Alice"), String::from("Bob")];

    // names.iter() 和 &names等价

    for name in names.iter() {
        println!("{}", name);
    }
}
#[derive(Debug)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[derive(Debug)]
pub struct Size {
    pub width: u8,
    pub height: u8,
}

#[derive(Debug)]
pub struct Car {
    pub color: Color,
    pub size: Size,
}

impl Car {
    pub fn new(color: Color, size: Size) -> Self {
        Car { color, size }
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn get_size(&self) -> &Size {
        &self.size
    }
}

pub fn test_impl() {
    let car = Car::new(
        Color {
            red: 255,
            green: 255,
            blue: 255,
        },
        Size {
            width: 1,
            height: 1,
        },
    );

    let width = car.get_size().width;

    println!("{:?}", car.get_color());
    println!("{:?}", car.get_size());
    println!("{}", width);
}

// 泛型
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_point(&self) -> &T {
        &self.x
    }
}

// 接口（抽象类），定义各个类的公共方法
trait Shape {
    type item; // 这个是接口的关联类型，可以在实现的时候指定为具体的类型
    fn get_area(&self) -> Self::item;
}

struct Circle {
    radius: f64,
}

// 给Circle实现Shape接口
impl Shape for Circle {
    type item = f64; // 实现接口的时候具化类型
    fn get_area(&self) -> Self::item {
        3.14 * self.radius * self.radius
    }
}

// 泛型约束,约束传递的参数需要实现PartialOrd + Copy
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 生命周期
fn get_max<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

// 悬垂引用
pub fn test_quote() {
    let string = String::from("hello!!!");
    let result: &str;
    {
        let string1 = String::from("world");
        // string1的生命周期比result，运行会报错
        // result = get_max(string.as_str(), string1.as_str());
    }

    // println!("最大长度是：{}", result);
}

// ？将 Result 的 Ok 值直接返回
pub fn test_read_file() {
    let path = env::current_dir().unwrap().to_str().unwrap().to_owned();

    // unwrap直接提取结果
    // println!("{:?}", path.to_str().unwrap());

    let mut f = File::open(format!("{}/src/main.rs", path)).unwrap();

    let mut ctx = String::new();

    f.read_to_string(&mut ctx).unwrap();

    println!("文件内容是：{}", ctx);
}

// 宏
macro_rules! test_macro {
    ($x:expr) => {
        $x * 2
    };
}

pub fn test_macro_fn() {
    println!("{}", test_macro!(2));
}

// 使用format!宏创建字符串

pub fn test_format() {
    let name = "Alice";
    let age = 18;

    let message = format!("{} is {} years old", name, age);

    println!("{}", message);

    // 输出：Alice is 18 years old
}

// HelloMacro 宏的实现逻辑
// 过程宏
// #[proc_macro_derive(HelloMacro)]
// pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
// 	let ast:DeriveInput = syn::parse(input).unwrap();
// 	impl_hello_macro(&ast)
// }

// fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {

//     let name = &ast.ident;
//     let gen = quote! {
// 				// 实现 HelloMacro 特征
//         impl HelloMacro for #name {
//             fn hello_macro() {
//                 println!("Hello, Macro! My name is {}!", stringify!(#name));
//             }
//         }
//     };
//     gen.into()
// }

// #[derive(HelloMacro)]
// struct MyStruct;

// 属性宏
// #[proc_macro_attribute]

// 函数宏
// #[proc_macro]