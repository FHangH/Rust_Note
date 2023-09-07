// rust 的 String 是通过 Vec<u8>封装
// len()

fn main() 
{
    let len = String::from("Test String").len();
    println!("{}", len);

    let str1 = "abc";
    for i in str1.bytes()
    {
        print!("{} ", i);
    }
    println!();
    for i in str1.chars()
    {
        print!("{} ", i);
    }
    println!();

    let str2 = "abc";
    let str2_1 = &str2[1..];
    println!("{}", str2_1);
}