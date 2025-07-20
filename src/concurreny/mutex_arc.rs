use std::{rc::Rc, sync::{Arc, Mutex}, thread};

fn main() {

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}",m);

    // sharing mutex b/w multiple threads

    // let counter = Mutex::new(0);
    // let mut handles = vec![];
    // for _ in 0..30 {
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();

    //         *num +=1; 
    //     });
    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap()); // error: counter ownership cannot be moved to multiple threads

    // example 3:  Rc is not thread safe

    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];

    // for _ in 0..10 {
    //     let c = Rc::clone(&counter); 
    //     let handle = thread::spawn(move || {
    //         let mut num = c.lock().unwrap();
    //         *num += 1;
    //     });

    //     handles.push(handle);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }


    // example 4: Atomic Refernce countung Arc<T>

   let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    } 

    println!("result {}", counter.lock().unwrap());

}
