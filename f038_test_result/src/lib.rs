// 可以不用 panic，使用result<T, E> 作为返回类型进行编写
#[cfg(test)]
mod test
{
    #[test]
    //#[ignore]
    fn it_work() -> Result<(), String>
    {
        if 2 + 2 == 4
        {
            return Ok(());
        }
        return Err("two number plus error".to_string());
    }
}

// 并行测试
// cargo test -- --test-threads=10

// 测试想看println!
// --show-output

// 按名称测试
// cargo test test
// cargo test it_work

// 忽略测试
// #[ignore = "reason"]
// cargo test -- --ignored

// 单元测试
// #[cfg(test)]

// 集成测试
// 先创建 tests 目录
// tests目录下的每一个测试文件都是单独的crate