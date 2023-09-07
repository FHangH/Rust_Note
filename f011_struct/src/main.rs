#[derive(Debug)]
struct Rectangle
{
    width: u32, 
    height: u32
}

fn build_rectangle(width: u32, height: u32) -> Rectangle
{
    return Rectangle
    {
        width, height
    };
}

fn calculate_area(rect: &Rectangle) -> u32
{
    return rect.width * rect.height;
}

fn main() 
{
    let rect = Rectangle
    {
        width: 30,
        height: 50
    };
    println!("area {}", calculate_area(&rect));

    let rect_1 = build_rectangle(10, 10);
    println!("area1 {}", calculate_area(&rect_1));

    // 通过格式化输出
    println!("#[derive(Debug)] {:?}", rect_1);
    println!("#[derive(Debug)] {:#?}", rect_1);
}