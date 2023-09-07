#[derive(Debug)]
pub struct Rectangle
{
    length: u32, width: u32
}

impl Rectangle
{
    pub fn can_hold(&self, other: &Rectangle) -> bool
    {
        return self.length > other.length && self.width > other.width;
    }
}

#[cfg(test)]
mod test
{
    //use super::*;
    use super::Rectangle;

    #[test]
    fn larger_can_hold_smaller() 
    {
        let larger = Rectangle
        {
            length: 8, width: 7
        };

        let smaller = Rectangle
        {
            length: 4, width: 2
        };
        assert!(larger.can_hold(&smaller))
    }

    #[test]
    fn samller_cannot_hold_larger() 
    {
        let larger = Rectangle
        {
            length: 8, width: 7
        };

        let smaller = Rectangle
        {
            length: 4, width: 2
        };
        assert!(!smaller.can_hold(&larger))
    }
}