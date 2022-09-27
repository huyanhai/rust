use std::collections::HashMap;

pub fn test_iterators() {
    let name = ["张三", "李四"];
    let age = [12, 20];
    let info: HashMap<_, _> = name.into_iter().zip(age.into_iter()).collect();

    println!("{:?}", info);
    // {"张三": 12, "李四": 20}

    #[derive(Debug)]
    struct Meters(u32);

    let m = Meters(20);

    println!("m is {:?}", m.0);
}
