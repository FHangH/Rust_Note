// Trait 告知编译器：莫种类型具有哪些，可以和其他类型共享的功能
// 抽象的定义共享功能
// Trait bounds ： 泛型类型参数指定为实现了特定行为的类型
// 类似 interface，但有区别

// 定义
// 方法签名写到一起，定义某种目的所必须的一组行为
// 只有方法签名（声明），没有实现

use f031_trait::Summary;
use f031_trait::Tweet;

fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T 
{
    let mut largest = &list[0];
    for item in list
    {
        if item > &largest
        {
            largest = item;
        }
    }
    return largest;
}

fn main() 
{
    let tweet = Tweet::new(
        "horse_ebooks".to_string(), 
        "of course, as you probably already know, people".to_string(), 
        false, false);
    println!("1 new tweet: {}", tweet.summarize());

    let str_list = vec!["Hello".to_string(), "world".to_string()];
    let result = largest(&str_list);
    println!("Largest word: {}", result);
}
