use std::{io, fs::File, io::Read, error::Error};

fn read_username_from_file() -> Result<String, io::Error>
{
    let f = File::open("hello.txt");
    let mut f = match f
    {
        Ok(file) => file,
        Err(e) => return Err(e)            
    };
    let mut s = String::new();
    match f.read_to_string(&mut s)
    {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    }
}

// 当 函数 只返回值，不处理错误，错误交给调用者处理，就可以这样写
// ？运算符 只能返回 Result Or Option
fn read_username_from_file_new() -> Result<String, io::Error>
{
    // let mut f = File::open("hello.txt")?;
    // let mut s = String::new();
    // f.read_to_string(&mut s)?;

    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    return Ok(s);
}

fn main() -> Result<(), Box<dyn Error>> 
{
    match read_username_from_file()
    {
        Ok(s) => println!("Ok {:#?}", s),
        Err(e) => println!("Err {:#?}", e)
    }
    
    match read_username_from_file_new()
    {
        Ok(s) => println!("New Ok {:#?}", s),
        Err(e) => println!("New Err {:#?}", e)
    }

    //File::open("hello.txt")?;
    return Ok(());
}
