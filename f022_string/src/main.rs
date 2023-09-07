// 字符型为 byte
// rust 核心层面， 只有一个字符串类型：字符切片：str
// String 是标准库的容器，其他字符串类型：OsString, OsStr, CString, CStr
// rust String 不支持下标索引

fn main() 
{
    let a = "aaa";
    let a_string = a.to_string();
    println!("a {}", a_string);

    let b = String::from("value");
    println!("b {}", b);

    let mut c = String::from("value");
    c.push_str("string");
    println!("c {}", c);

    let mut d = String::new();
    d.push_str("string");
    println!("d {}", d);

    // push 是传 char
    // push_str 是传 str
    let mut e = String::new();
    e.push('e');
    e.push('e');
    e.push_str("string");
    println!("e {}", e);

    // + 可以拼接 string
    // + 重载了 类似于重载函数 fn add(self, s:&str) -> String{}
    let f = b + &c;
    println!("f {}", f);

    // format!();
    // 格式化 拼接字符串
    let g = format!("{} - {}", c, d);
    println!("g {}", g);
}