//use std::fmt::{Display, Debug};

pub enum Type
{
    NA, TW
}

pub trait Summary 
{
    fn summarize(&self) -> String;    
}

pub struct NewArticle
{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl NewArticle
{
    pub fn new(headline: String, location: String, author: String, content: String) -> Self
    {
        return Self { headline: headline, location: location, author: author, content };
    }    
}

pub struct Tweet
{
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Tweet 
{
    pub fn new(username: String, content: String, reply: bool, retweet: bool) -> Self
    {
        return Self { username: username, content: content, reply:reply, retweet: retweet };
    }    
}

impl Summary for NewArticle
{
    fn summarize(&self) -> String 
    {
        return format!("{}, by {} ({})", self.headline, self.author, self.location);
    }
}

impl Summary for Tweet
{
    fn summarize(&self) -> String 
    {
        return format!("{}: {}", self.username, self.content);
    }    
}

pub fn notify<T: Summary>(item: T) -> String
{
    return format!("Notify: {}", item.summarize());
}

// // 下面的写法 可以在 泛型的基础上进行约束
// pub fn notify_1<T: Summary + Display, U:Clone + Debug>(a: T, b: U) -> String
// {
//     return format!("Notify_1: {}", a.summarize());
// }

// pub fn notify_2<T, U>(a: T, b: U) -> String
// where T: Summary + Display, U:Clone + Debug
// {
//     return format!("Notify_2: {}", a.summarize());
// }

// // 这样写 返回的 trait 只是 NewArticle 类型
// // impl Summary 只能返回一种类型
// pub fn notify_3(t: &Type) -> impl Summary
// {
//     match t 
//     {
//         NA => 
//         {
//             return NewArticle::new(
//                 "headline".to_string(), 
//                 "location".to_string(), 
//                 "author".to_string(), 
//                 "content".to_string());
//         },
//         TW => 
//         {
//             return Tweet::new(
//                 "username".to_string(),
//                 "content".to_string(),
//                 false,
//                 false);
//         }
//     }
// }