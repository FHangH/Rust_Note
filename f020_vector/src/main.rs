fn main() 
{
    let mut v1: Vec<u32> = Vec::new();
    v1.push(1);
    // 支持上下文 类型推导
    // let mut v1 = Vec::new();
    // v1.push(1);

    // vec![] 自动推导类型
    let v2 = vec![1, 2, 3, 4];
    let third = &v2[2];
    println!("Third element {}", third);

    match v2.get(2) 
    {
        Some(third) => println!("Third element {}", third),
        None => println!("No third element")
    }

    // for each vector
    let v3 = vec![1, 2, 3, 4];
    for i in &v3
    {
        print!("{} ", i);
    }
    println!();

    // for each vector and update elements
    let mut v4 = vec![1, 2, 3, 4];
    for i in &mut v4
    {
        // * 是解引用
        *i += 10;
        print!("{} ", i);
    }
    println!();
}
