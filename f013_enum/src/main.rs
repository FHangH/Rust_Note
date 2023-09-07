enum IpType
{
    IPV4, IPV6
}

fn ip_info(ip: &IpType)
{
    match ip 
    {
        IpType::IPV4 => {println!("IPV4")},
        IpType::IPV6 => {println!("IPV6")}
    }
}

#[derive(Debug)]
enum StudentType
{
    SCORE(u8, u8, u8),
    NAME(String)
}

fn main() 
{
    let ip4 = IpType::IPV4;
    let ip6 = IpType::IPV6;
    ip_info(&ip4);
    ip_info(&ip6);

    let stu_score = StudentType::SCORE(10, 20, 30);
    let stu_name = StudentType::NAME(String::from("qq"));
    println!("Student Type: {:?}", stu_score);
    println!("Student Type: {:?}", stu_name);
}
