fn take_ownership(str: String)
{
    println!("take_ownership {}", str);
}

fn makes_copy(number: u32)
{
    println!("make_copy {}", number);
}

////////////////////////////////
fn gives_ownership() -> String
{
    let str = String::from("hello");
    return str;
}

fn takes_and_gives_back(str: String) -> String
{
    return str;
}

fn main() 
{
    let s = String::from("hello world");
    take_ownership(s);
    
    let x = 5;
    makes_copy(x);

    println!("x {}", x);

    ////////////////////////////////
    let s1 = gives_ownership();
    let s2 = String::from("hello world");
    let s3 = takes_and_gives_back(s2.clone());
    println!("s1, s2, s3 {} {} {}", s1, s2, s3);
}
