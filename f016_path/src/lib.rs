fn server_order(){}

#[allow(dead_code)]
mod back_of_house
{
    fn fix_incorrect_order()
    {
        cook_order();
        super::server_order();
        crate::server_order();
    }

    fn cook_order(){}
}