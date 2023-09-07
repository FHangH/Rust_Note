// 生命周期标准法
// 不会改变生命周期
// 指定泛型生命周期参数，函数可以接受任意长度的生命周期
// 描述了多个引用的生命周期关系
// &i32 一个引用； &'a i32 有显式声明周期的引用
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str 
{
    if x.len() > y.len()
    {
        return x;
    }
    return y;
}

struct Test<'a>
{
    part: &'a str
}

// 特定情况下可以不用
// 后期rust编译器会慢慢取消 生命周期标准的写法

// 1.引用参数的传入值有独立的生命周期
// 2.如果只有一个输入生命周期参数，那么该生命周期被赋给所有的输出的生命周期参数
// 3.如果有多个，其中一个是&self或 &mut self，那么self的生命周期赋给所有的输出生命周期参数
fn first_word(s: &str) -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    return &s[..];
}

// 'a 可以当 泛型
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: Display
{
    println!("Announcement:{}", ann);
    if x.len() > y.len()
    {
        return x;
    }
    return y;
}

fn main() 
{
    let str1 = "hello world".to_string();
    let str2 = "hello";
    let result = longest(str1.as_str(), str2);
    println!("{}", result);

    let novel = "hello. world".to_string();
    let first_sentence = novel.split('.').next().expect("couldn't find '.'");
    let i = Test{ part: first_sentence };
    println!("{}", i.part);

    let test = "hello world";
    let test1 = first_word(test);
    println!("{}", test1);

    let s1 = "hello world";
    let s2 = "hello";
    let s3 = longest_with_an_announcement(s1, s2, "test");
    println!("{}", s3);
}
