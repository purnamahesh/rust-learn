// use std::

use std::{thread, time::Duration};

fn main() {
    let v =vec![1,2,3];

    let x = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
            println!("{:?}",v);
        }
    });
    
    // x.join().unwrap();
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    
    x.join().unwrap();

}
