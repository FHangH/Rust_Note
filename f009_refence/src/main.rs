fn calculate_length(s: &String) -> usize 
{
    return s.len();
}

fn main() 
{
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);
    println!("length of '{}' is {}", s1, len);
}
