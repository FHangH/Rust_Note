use std::env;
use std::process;
use f039_grep::Config;
use f039_grep;

fn main() 
{
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| 
    {
        println!("Error arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = f039_grep::run(&config)
    {
        println!("Error: {}", e);
        process::exit(1);
    }
}
