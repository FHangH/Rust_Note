struct Point<T>
{
    x: T, y:T
}

impl<T> Point<T>
{
    fn new(x: T, y: T) -> Point<T>
    {
        return Point { x: x, y: y };
    }

    fn x(&self) -> &T
    {
        return &self.x;
    }

    fn y(&self) -> &T
    {
        return &self.y;
    }
}

fn main() 
{
    let p = Point::new(0.0, 0.0);
    println!("({}, {})", p.x(), p.y());
}
