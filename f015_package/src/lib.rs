#[allow(dead_code)]
mod front_of_house
{
    // pub 修饰为 public
    pub mod hosting
    {
        pub fn add_to_waitlist(){}
        fn seat_at_table(){}
    }

    // 默认是 private
    mod serving
    {
        fn take_order(){}
        fn server_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant()
{
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}