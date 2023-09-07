fn main() 
{
    let conditions = true;
    // if else
    let result = if conditions {5} else {10};
    println!("result {}", result);

    // rust 的 if else 一般可以用match代替，类似与switch
    let result_1 = match conditions
    {
        true => 5,
        false => 10
    };
    println!("result_1 {}", result_1);

    // loop 程序无条件反复执行，直到手动跳出
    let mut number = 0;
    loop
    {
        println!("loop {}", number);

        if number > 5 {break};
        number += 1;
    }

    // while 条件循环
    while number > 0
    {
        number -= 1;
        println!("while {}", number);
    }

    // for 借助迭代器进行遍历
    let arr: [u8; 5] = [0, 1, 2, 3, 4];
    for ele in arr.iter()
    {
        println!("for {}", ele);
    }

    // for range 左闭右开
    for ele in 1..3
    {
        println!("for range(1..3) {}", ele);
    }

    for ele in 1..=3
    {
        println!("for range(1..=3) {}", ele);
    }

    for ele in (1..3).rev() // rev 翻转
    {
        println!("for range.rev(1..3) {}", ele);
    }
}
