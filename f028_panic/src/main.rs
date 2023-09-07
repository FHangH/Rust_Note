pub struct Guess
{
    value: i32
}

impl Guess 
{
    pub fn new(value: i32) -> Guess
    {
        if value < 1 || value > 100
        {
            panic!("Input {}, only recive [1, 100]", value);
        }
        return Guess{ value };
    }    

    pub fn value(&self) -> i32 { return self.value; }
}

fn main() 
{
    loop 
    {
        let guess = "999";
        let guess: i32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue
        };

        let g = Guess::new(guess);
        println!("{:#?}", g.value);
    }
}
