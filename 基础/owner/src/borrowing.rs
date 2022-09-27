/**
 * 引用和借用
 */

// 获取变量的引用叫借用
fn test1() {
    let x = 10;
    let y = &x;

    println!("{},{},{}", x, y, *y);
    // assert_eq!(10, y); 类型不匹配`{integer} == &{integer}`
}

// 不可变引用
fn test2(str: &String) {
    // str.push_str(' 123');由于str不可变，所以不能复制
    println!("str is :{}", str);
}

// 可变引用
fn test3(str: &mut String) {
    println!("str1 is :{}", str)
}

pub fn test_borrowing() {
    test1();

    let str = String::from("my_str");
    test2(&str);
    println!("{}", str);

    let mut str1 = String::from("my_str1");
    test3(&mut str1); // 传入可变类型的str1

    // 可变引用只能同时存在一个
    let _s1 = &mut str1;
    // let s2 = &mut str1;

    println!("{}", str1);

    // 可变引用和不可变引用不能同时存在
    let mut str2 = String::from("my_str2");

    let s2_1 = &str2;
    let s2_2 = &str2;
    println!("{},{}", s2_1, s2_2);

    let s2_3 = &mut str2;
    // let s2_4 = &mut str2;
    println!("{}", s2_3);
}

// 悬垂引用 编译报错
// pub fn test_dr() -> &String {
//                     ^ this function's return type contains a borrowed value, but there is no value for it to be borrowed from
//                       help: consider using the `'static` lifetime: `&'static `
//     let str = String::from("my_test_dr");
//     return &str;
// }
