mod points;
use points::*;

fn main() {
    // 静态生命周期 'static
    // 和整个程序一样长

    let sa = String::from("xxx");

    let s1 = &sa;
    let s2 = &sa;

    println!("{},{},{}", s1, s2, sa);

    let _s1 = String::from("静态生命周期");

    test_point();

    let long = String::from("这是一个比较长的字符串");
    let _result;
    {
        let short = String::from("短的字符串");
        _result = largest(long.as_str(), short.as_str());
    }

    let _p: Point;
    {
        let x = 20;
        _p = Point {
            x: &x,
            name: String::from("xxx"),
        };
    }
}
