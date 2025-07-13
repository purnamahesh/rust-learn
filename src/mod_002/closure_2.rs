use std::{thread, time::Duration};

use rand::rand_core::UnwrapErr;

fn main () {
    let expensive_closure = |x: u32| -> u32 {
        thread::sleep(Duration::from_secs(1));
        x
    };

    fn add_one_v1(x:u32) -> u32 { x+1 }

    let add_one_v2 = |x: u32| -> u32 { x + 1};

    let add_one_v3 = |x| { x + 1};

    let add_one_v4 = |x:u8|  x + 1;

    let x = add_one_v3(5);


    // example 2 - immutable borrow

    let list = vec![1,2,3];

    println!("before defining closure {:?}", list);

    let only_borrows =|| println!("from closure {:?}", list);

    println!("before calling closure {:?}", list);
    only_borrows();
    println!("after calling closure {:?}", list);

    // example 2 - mutable borrow

    let mut list = vec![1,2,3];

    println!("before defining closure {:?}", list);

    let mut borrows_mutubly =|| list.push(7);

    borrows_mutubly();

    // println!("after calling closure {:?}", list); // fails
    // borrows_mutubly();


    // example 3 - taking (moving) ownership

    let list =  vec![1,2,3];
    println!("before defining closure {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
    .join()
    .unwrap();

    // .unwrap_or_else(Vector::new);

}


