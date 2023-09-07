// 测试除了验证代码返回值是否正确，还需验证代码是否如预期处理了错误
#[allow(dead_code)]
pub struct Guess
{
    value: u32
}

impl Guess 
{
    pub fn new(value: u32) -> Guess
    {
        if value < 1 || value > 100
        {
            panic!("Invalid guess value {}", value);
        }
        return Guess { value: value }
    }    
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    //#[should_panic]
    #[should_panic(expected = "Invalid guess must be between 1 and 100")]
    fn greater_than_100()
    {
        Guess::new(50);
    }
}