use std::{sync::mpsc, thread, time::Duration};



fn main() {

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); error: giving up ownership in the above line
    });

    let received = rx.recv().unwrap();


    println!("Got: {}", received);

    // example 2

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("there!"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for val in rx {
        println!("Got from thread: {}", val);
    }

    println!("\n\n\n\n");

    // example 3: multiple producers

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
       let vals = vec![
            String::from("Hi"),
            String::from("there!"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        } 
    });

    thread::spawn(move || {
       let vals = vec![
            String::from("from"),
            String::from("the"),
            String::from("second"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(500));
        } 
    });

    for val in rx {
        println!("{}", val);
    }

}
