mod back_of_house
{
    // struct 自身声明需要 pub 修饰， 其子项也需要 pub 修饰
    // enum 自身声明 pub 修改后，其子项也均为 pub
    #[allow(dead_code)]
    pub struct Breakfast
    {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast
    {
        pub fn summer(toast: &str) -> Breakfast
        {
            return Breakfast 
            { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches") 
            }
        }
    }
}

pub fn eat_at_restaurant()
{
    let mut meal = crate::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}