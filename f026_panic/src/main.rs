// panic
//  - 程序展开调用栈（工作量大），rust沿着调用栈返回，清理数据
//  - 程序暂停，rust不进行清理，内存由OS进行清理

// 展开 -> 中止：二进制文件更小
// 在 Cargo.toml 中的 profile 进行设置
// panic = "abort"
// 在本工程的 .toml 中查看设置方法

use std::{fs::File, io::ErrorKind};

fn main() 
{
    //panic!("===== Test Panic ======")

    let f = File::open("hello.txt");
    match f 
    {
        Ok(file) => file,
        Err(error) => match error.kind()
        {
            ErrorKind::NotFound => match File::create("hello.txt") 
            {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),    
            },
            other_error => panic!("Error open file: {:?}", other_error)
        }
    };

    // 闭包
    File::open("hello.txt").unwrap_or_else(|error|
    {
        if error.kind() == ErrorKind::NotFound
        {
            File::create("hello.txt").unwrap_or_else(|error|
            {
                panic!("Error creating file: {:?}", error);
            })
        }
        else 
        {
            panic!("Error opening file: {:?}", error);    
        }
    });
}