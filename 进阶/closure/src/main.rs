mod closures;
mod iterators;

use closures::test_closure;

fn main() {
    test_closure();
    iterators::test_iterators();

    // !永不返回类型
    // panic 的返回值是 !
}
