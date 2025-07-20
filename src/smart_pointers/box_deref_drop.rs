



// cons list: recersive type
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

// defining own smart pointers
struct MyBox<T>(T);

impl<T> MyBox<T>  {
    fn new(x:T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>  {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T>  {
    fn drop(&mut self) {
        println!("Dropping MyBox with data!");
    }
}

use std::ops::Deref;

use crate::List::{Cons, Nil};

fn main () {

    let b = Box::new(5);
    println!("b = {b}");

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:#?}", list);

    let a = 1;
    let b = &a;
    assert_eq!(a, *b);

    let a = 5;
    let b = Box::new(a);
    assert_eq!(a, *b);

    // early drop
    drop(b);

    let a = 5;
    let b = MyBox::new(a);
    assert_eq!(a, *b); // this fails since MyBox not implementing DeRef


    // deref coercions

    drop(b);
}

// TODO: Box is used to allocate memory in heap (instead of stack) and to create recersive types