#[test]
fn test_1()
{
    assert_eq!(2 + 2, 4);
}

#[test]
fn test_2()
{
    panic!("test_2 error");
}

// cargo test
// 运行测试