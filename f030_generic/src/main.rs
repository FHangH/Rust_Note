struct Point<T, U>
{
    x: T, y: U
}

impl<T, U> Point<T, U>
{
    fn new(x: T, y: U) -> Point<T, U>
    {
        return Point { x: x, y: y };
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W>
    {
        return Point { x: self.x, y: other.y };
    }
}

fn main() 
{
    let p1 = Point::new(1, 2);
    let p2 = Point::new("a", 'b');
    let p3 = p1.mixup(p2);

    println!("(x:{}, y:{})", p3.x, p3.y);
}