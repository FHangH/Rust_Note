use std::collections::HashMap;

fn main() 
{
    // heap 上
    let mut map: HashMap<i32, String> = HashMap::new();
    map.insert(0, String::from("qq"));

    // define hashmap
    let colors = vec![String::from("Green"), String::from("Red")];
    let scores = vec![10, 20];
    let cs_hash: HashMap<_, _> = colors.iter().zip(scores.iter()).collect();
    // foreach
    for i in cs_hash.iter()
    {
        print!("for iter {}-{}\n", i.0, i.1);
    }

    for (k, v) in &cs_hash
    {
        print!("for (k,v) {}-{}\n", k, v);
    }

    for i in &cs_hash
    {
        print!("for i {}-{}\n", i.0, i.1);
    }

    // hashmap 会获取 其他的所有权
    let a = String::from("a");
    let b = String::from("b");
    let mut c_map = HashMap::new();
    c_map.insert(a, b);
    // 报错， 提示a b 已经被 moved
    //println!("{}-{}", a, b);

    let c = String::from("c");
    let d = String::from("d");
    let mut e_map = HashMap::new();
    e_map.insert(&c, &d);
    println!("{}-{}", c, d);

    // match hashmap
    let mut m_map = HashMap::new();
    m_map.insert(10, 100);
    m_map.insert(20, 200);

    let mk = 10;
    let mv = m_map.get(&mk);
    match mv 
    {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
}