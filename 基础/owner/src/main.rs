mod borrowing;
mod owner;

fn main() {
    // 所有权test
    owner::test_owner();
    println!("-------------");
    // 引用和借用test
    borrowing::test_borrowing();
}
