// 数组

pub fn test_arr() {
    // 定义一个定长的数组
    let arr = [1, 2, 3, 4, 5];

    // 定义一个动态数组
    let mut arr_1 = Vec::new();

    arr_1.push(1);

    println!("{:?},{:?}", arr, arr_1);
}
