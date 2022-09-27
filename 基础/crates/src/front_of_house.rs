fn test_root() {}

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            // 调用父模块
            super::test_super();
            // 调用父类的父类的模块
            super::super::test_root();

            // 调用同级模块
            self::test_self();

            let s: super::test_struct = super::test_struct {
                name: String::new(),
            };

            println!("{:?},{:?}", s.name, super::test_enum::Black);
        }

        pub fn test_self() {}
    }

    // 将结构体设置为 pub，但它的所有字段依然是私有的
    #[derive(Debug)]
    pub struct test_struct {
        name: String,
    }

    // 将枚举设置为 pub，它的所有字段也将对外可见
    #[derive(Debug)]
    pub enum test_enum {
        Red,
        Black,
    }

    pub fn test_super() {}
}
