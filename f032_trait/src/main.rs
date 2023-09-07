// 在使用泛型类型参数的impl 上使用 trait bound，可以有条件的实现特点 trait的类型来实现方法

use std::fmt::Display;

struct Pair<T>
{
    x: T, y: T
}

impl<T> Pair<T>
{
    fn new(x: T, y: T) -> Self
    {
        return Self{ x, y };
    }
}

// T: Display + PartialOrd 是约束
// 目的是保证 T 是支持 Display，PartialOrd 两种功能的类型
impl <T: Display + PartialOrd> Pair<T>
{
    fn cmp_display(&self)
    {
        if self.x >= self.y
        {
            println!("largest: {}", self.x);
            return;
        }
        println!("largest: {}", self.y);
    }
}

fn main()
{
    let pair = Pair::new(10, 20);
    pair.cmp_display();
}