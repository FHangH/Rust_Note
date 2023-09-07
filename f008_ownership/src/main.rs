fn calculate_length(s: String) -> (String, usize)
{
    let length = s.len();
    return (s, length);
}

fn main() 
{
    let s1 = String::from("hello world");
    let (s2, len) = calculate_length(s1);
    println!("length of '{}' is {}", s2, len);
}
