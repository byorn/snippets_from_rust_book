//A trait defines the functionality a particular
// type has and can share with other types.
// We can use traits to define shared behavior in
// an abstract way. We can use trait bounds to specify
// that a generic type can be any type that has certain
//  behavior.

//Note: Traits are similar to a feature often called interfaces
// in other languages, although with some differences./
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait SummaryDefault {
    fn summarize1(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl SummaryDefault for NewsArticle{}

pub struct SocialPost {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub repost: bool,
}

impl Summary for SocialPost {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn test_traits(){
    let post = SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        };

        println!("1 new post: {}", post.summarize());
    println!("ok");
}

pub fn test_defaulttraits(){
    let post = NewsArticle {
author: String::from("enid blyton"),
content: String::from("book"),
headline: String::from("title"),
location: String::from("kandy")
        };

        println!("1 new post: {}", post.summarize1());
    println!("ok");
}

/*
 *traits as parameter
 *
 * pub fn notify(item: &impl Summary) {
     println!("Breaking news! {}", item.summarize());
 }


same as


pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
 *
 *
 *
 */



 /*
  *   more tips and tricks
  *
  * see :   Multiple Trait Bounds with the + Syntax
  *
  *
  * see:  Clearer Trait Bounds with where Clauses
  *
  *
  *
  */
