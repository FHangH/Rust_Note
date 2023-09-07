use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() 
{
    let secret_number = rand::thread_rng().gen_range(0..=100);
    
    loop 
    {
        println!("Guess Number");
        let mut guess_number = String::new();

        io::stdin().read_line(&mut guess_number).expect("Error reading");
        
        let guess_number: u32 = match guess_number.trim().parse()
        {
            Ok(number) => number,
            Err(_) => continue
        };
        println!("Input Number {}", guess_number);
    
        match guess_number.cmp(&secret_number)
        {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => 
            {
                println!("Guess Equal");
                break;
            }
        }    
    }
}
