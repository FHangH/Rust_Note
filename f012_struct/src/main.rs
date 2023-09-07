#[derive(Debug)]
struct Rectangle
{
    width: u32, 
    height: u32
}

impl Rectangle
{
    fn area(&self) -> u32 
    { 
        return self.width * self.height;
    }

    fn build_from_rect(&mut self, rect: &Rectangle)
    {
        self.width = rect.width;
        self.height = rect.height;
    }
}

fn main() 
{
    let rect_1 = Rectangle
    {
        width: 30,
        height: 30, 
    };
    println!("react_1 area {}", rect_1.area());

    let rect_2 = Rectangle
    {
        width: 100,
        height: 100
    };
    let mut rect_3 = Rectangle{width: 0, height: 0};

    rect_3.build_from_rect(&rect_2);

    println!("react_3 area {}", rect_3.area());
}