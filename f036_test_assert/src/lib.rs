// assert!  assert_eq!  assert_ne!
// assert! 第一参数必填，第二参数为自定义 std::fmt
// assert_eq! assert_ne! 前两个参数必填，第三参数为自定义 std::fmt
// 自定义消息会被传递给 format!， 可以使用 {}

pub fn greeting(name: &str) -> String
{
    return format!("hello {}", name);
}

pub fn greeting_1(_name: &str) -> String
{
    return format!("=====");
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn greeting_contain_name()
    {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }

    #[test]
    fn greeting_contain_name_1()
    {
        let result = greeting_1("Carol");
        assert!(
            result.contains("Carol"), 
            "Greeting_1 did not contain Carol, last name: {}", result);
    }
}