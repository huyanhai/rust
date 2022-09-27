// 动态数组

fn vec1() {
    let mut arr: Vec<i32> = Vec::new();

    arr.push(1);
    arr.push(2);

    println!("arr is :{:?}", arr);
}

fn vec2() {
    let arr = vec![1, 2, 3, 4];

    // 读取值

    let n = arr.get(1);

    println!("arr is :{:?},n is :{:?}", arr, n);
}

pub fn text_vec() {
    vec1();
    vec2();
}
