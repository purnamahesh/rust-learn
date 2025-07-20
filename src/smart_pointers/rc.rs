use std::rc::Rc;

enum List2 {
    Cons(i32, Box<List2>),
    Nil,
}

// cons list: recersive type
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}



fn main () {

    use crate::List2::{Cons as Cons2, Nil as Nil2};

    let a = Cons2(5, Box::new(Cons2(10, Box::new(Nil2))));
    let b = Cons2(3, Box::new(a));
    // value moved here
    // move occurs because `a` has type `List2`, which does not implement the `Copy` trait
    // let c = Cons2(4, Box::new(a)); //ERROR:

    use crate::List::{Cons, Nil}; 

    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    let c = Rc::new(Cons(4, Rc::clone(&a)));


    // Example: Reference Counts
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
