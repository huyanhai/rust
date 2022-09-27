// HashMap

use std::collections::HashMap;

pub fn test_hmp() {
    let mut hmp = HashMap::new();

    hmp.insert("name", "张三");
    //如果存在name则返回name，如果没有则重新赋值在返回

    let person = vec![[("张三".to_string(), 10), ("李四".to_string(), 20)]];

    // let p: HashMap<_, _> = person.into_iter().collect();

    println!("hmp is :{:?},{:?}", hmp, hmp.get("name"));

    let new_name = hmp.entry("name").or_insert("李四");

    println!("new_name is {:?}", new_name);
}
