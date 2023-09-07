fn main() 
{
    let mut str1 = String::from("hello ");
    str1.push_str("world");
    println!("{}", str1);

    let str2 = "A".to_string();
    println!("{}", str2);

    let s1 = String::from("Hello");
    //let s2 = s1; //s1 已经被释放了, 类似浅拷贝，但源被释放
    println!("{}", s1);

    let s3 = String::from("Hello");
    let s4 = s3.clone(); // 类似深拷贝
    println!("{}", s3);
    println!("{}", s4);

    // 任何简单标量的组合类型都可以是 copy
        // u32, i32, f32, f64, bool, char... 所有字段都是copy 的 tuple
    // 任何需要分配内存或某种资源的都不是 copy
}
