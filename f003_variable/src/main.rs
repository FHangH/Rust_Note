const MAX: u32 = 9999;

fn main() 
{
    ////////////////////////////////
    let number = 100;
    println!("{}", number);

    let mut number = 1; //此时前面的 number被隐藏，后面默认都是 mut number
    number = number + 20;

    let mut new_number = number + MAX;
    new_number += MAX;

    println!("{}", MAX);
    println!("{}", number);
    println!("{}", new_number);

    ////////////////////////////////
    let a:u8 = 0; // u8 8bit => u16, u32, u64, usize: 依据操作系统位数
    let b:i8 = 0; // i8 8bit => i16, i32, i64, isize: 依据操作系统位数
    println!("{} {}", a, b);

    ////////////////////////////////
    let c:f32 = 0.0;
    let d:f64 = 10.0;
    println!("{} {}", c, d);

    ///////////////////////////////
    let c1: char = 'c'; // unicode, 32bit
    let c2: char = '🤣';
    println!("{} {}", c1, c2);

    //////////////////////////////
    let tp: (u32, i32, f32) = (10, 100, 10.0); // tuple
    let (x, y, z) = tp;
    println!("{} {} {}", tp.0, tp.1, tp.2);
    println!("{} {} {}", x, y, z);

    /////////////////////////////
    let arr1: [u8; 7] = [0, 1, 2, 3, 4, 5, 6];
    let arr2 = [3; 3]; // let arr2: [u8; 3] = [3, 3, 3]
    println!("{} {}", arr1[1], arr2[0]);
}
