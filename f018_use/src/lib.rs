mod front_of_house
{
    pub mod hosting
    {
        pub fn add_to_waitlist(){}

        #[derive(Debug)]
        pub struct Test{}
    }

    pub mod testing
    {
        #[derive(Debug)]
        pub struct Test{}
    }

    pub mod testing2
    {
        #[derive(Debug)]
        pub struct Test{}
    }
}

use crate::front_of_house::hosting;
use std::collections::HashMap;
use crate::front_of_house::testing;
use crate::front_of_house::testing2::Test as t_test;
// 如果 外部文件想使用这个文件的 use，可以在 use 前利用 pub 修饰

// 这样写可以直接用 add_to_waitlist()
//use crate::front_of_house::hosting::add_to_waitlist;

// 这样写可以直接用 hosting 里面所有的 第一级子项
//use crate::front_of_house::hosting::*;

// 注意：
//  - 一般 function 指定到父级最佳 ： use crate::front_of_house::hosting;
//  - enum, struct 指定到自身最佳： use std::collections::HashMap;
//  - 同名条目，指定到父级最佳
//      - 如果一定要 指定到自身，可以通过 as 进行 别名： use crate::front_of_house::testing2::Test as t_test;
pub fn eat_at_restaurant()
{
    hosting::add_to_waitlist();
    //add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    let test1 = hosting::Test{};
    let test2 = testing::Test{};
    let test3 = t_test{};
    println!("{:?}", test1);
    println!("{:?}", test2);
    println!("{:?}", test3);
}

// use std::io;
// use std::io::Write;
// 等价于 下面的写法
// use std::io::{self, Write};