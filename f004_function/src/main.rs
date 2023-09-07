fn test()
{
    println!("Test Function");
}

fn test1(x: u32)
{
    println!("Test Function {}", x);
}

fn test2(x: u32) -> u32
{
    x + 5 // return x + 5;
}

fn test3(x: u32) -> u32
{
    return x + 10;
}

fn main() 
{
    test();
    test1(10);
    println!("Test Function {}", test2(10));
    println!("Test Function {}", test3(10));
}
