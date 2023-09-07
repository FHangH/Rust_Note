fn first_word(s: &String) -> &str
{
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    return &s[..];
}

// 字符串切片作为参数，功能一样，但更加通用
fn first_word_new(s: &str) -> &str
{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return &s[..i];
        }
    }
    return &s[..];
}

fn main() 
{
    let s = String::from("hello world");
    let hello = &s[..5]; // [0..5] || [0..=4]
    let world = &s[6..]; // [6..11] || [6..s.len()]
    let all = &s[..]; // [0..s.len()]
    println!("{} {}", hello, world);
    println!("all {}", all);

    let str = String::from("hello world");
    let word_index = first_word(&str);
    println!("{}", word_index);

    let test_str_1 = String::from("hello world");
    println!("String::form {}", first_word_new(&test_str_1));
    println!("const String {}", first_word_new("hello world"));
}
