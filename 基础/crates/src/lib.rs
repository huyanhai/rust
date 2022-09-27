mod front_of_house;

pub use crate::front_of_house::front_of_house::hosting as test;

// 引入更多的模块
// use std::io::{self, Write};
// use std::collections::*;

pub fn eat_at_restaurant() {
    // 绝对路径
    test::add_to_waitlist();

    // 相对路径
    front_of_house::front_of_house::hosting::add_to_waitlist();
}
