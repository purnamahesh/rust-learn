// traits
// defining and implementing
// default implementations
// traits as parameters: trait bound
// &impl and generics syntax
// multiple trait bounds with +
// where clause
// returing types that implememnts traits
// conditional trait bounds


use std::fmt::{Debug, Display};


pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Repr  {
    fn to_string(&self) -> String {
        String::from("this is not overwritten yet!")
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

struct NewsArticle {
    headline: String,
    author: String,
    content: String,
    location: String,
    date: String
}


impl Summary for NewsArticle   {
    fn summarize(&self) -> String {
        format!(
            "{}, by {} ({}, {})", 
            self.headline, 
            self.author,
            self.location,
            self.date
        )
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!(
            "{}: {}",  
            self.username, 
            self.content
        )
    }
}

impl NewsArticle { 
    // TODO: trait bound: accepts any struct or enum that impls Summary trait
    fn notify(item: &impl Summary) {
        println!("BreAKING NEWSS! {}", item.summarize());
    }

    // TODO: trait bound alternatiev syntax: accepts any struct or enum that impls Summary trait
    fn notify_2<T: Summary>(item: &T) {
        println!("BreAKING NEWSS! {}", item.summarize());
    }

    fn notify_3<T: Summary + Repr>(item: &T) {
        println!("BreAKING NEWSS! {}", item.summarize());
    }

    fn notify_4(item: &(impl Summary + Repr)) {
        println!("BreAKING NEWSS! {}", item.summarize());
    }

    // clearer trait bounds with where
    // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

    fn notify_5<T, U>(t: &T, u: &U) -> i32 
    where 
        T: Display + Clone,
        U: Clone + Debug
    {
        0i32
    } 
}

impl Repr for Tweet {
    fn to_string(&self) -> String {
        format!(
            "{}: {}\n\nreply:{}\nretweet{}", 
            self.username, 
            self.content,
            self.reply,
            self.retweet
        )
    }
}

fn return_summarizable() -> impl Summary { // only one retuen type (NewsArticle or Tweet) is possible
    Tweet {
        username: String::from("elon_musk"),
        content: String::from("To the moon!"),
        reply: true,
        retweet: true
    }
}


fn main () {
    let na = NewsArticle {
        author: String::from("Frank Herbert"),
        headline: String::from("Sands of Dune"),
        content: String::from("The sands of Dune of very hot"),
        location: String::from("Sahara"),
        date: String::from("1979-10-20")
    };

    let na_summary = na.summarize();

    println!("{na_summary}");

    // NewsArticle::notify(&na);

    let ts = Tweet {
        username: String::from("elon_musk"),
        content: String::from("To the moon!"),
        reply: true,
        retweet: true
    };

    println!("{}",ts.summarize());

}
