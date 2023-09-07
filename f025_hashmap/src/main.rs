use std::collections::HashMap;

fn main() 
{
    // hash 大小可变
    // k v 1:1
    // hash update, k 存在
    //  - 替换 v
    //  - 保留旧 v
    //  - 合并 v

    // entry 检查 k 是否对应 v
    let mut score = HashMap::new();
    score.insert(String::from("a"), 10);
    
    let e = score.entry(String::from("b"));
    println!("{:?}", e);

    e.or_insert(50);
    score.entry(String::from("a")).or_insert(50);
    println!("{:?}", score);

    ////////////////////////////////
    let text = "Hello world Hello";
    let mut map = HashMap::new();
    for word in text.split_whitespace()
    {
        // 第一次 Hello 进来，map里面没有，添加进去，or_insert(0) => v = 0
        // or_insert() 返回 &mut 的 v

        // 第二次 Hello 进来，map里以及有了，不添加进去，但 or_insert()被调用 => v不变
        // 返回 &mut v
        let count = map.entry(word).or_insert(0);
        // 这里拿到 &mut v，解引用可以修改值
        *count += 1;
    }
    println!("{:#?}", map);
}
