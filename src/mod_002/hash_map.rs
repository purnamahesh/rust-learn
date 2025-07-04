use std::{collections::HashMap, hash::Hash, str::Chars};

fn main () {

    let mut hm = HashMap::new();

    hm.insert(String::from("Python"), 10);
    hm.insert(String::from("Java"), 5);

    let mut hm2 = HashMap::new();

    hm2.insert(10, String::from("Python"));
    hm2.insert(20, String::from("Java"));

    let xx = match hm2.get(&10) {
        Some(n) => n,
        None => &String::new()
    };

    let score = hm.get(&String::from("Python")).copied().unwrap_or(0);


    for (key, value) in &hm {
        println!("{key}: {value}");
    }


    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // update value

    hm.insert(String::from("Java"), 4);

    for (key, value) in &hm {
        println!("{key}: {value}");
    }

    hm.entry(String::from("Python")).or_insert(10);

    hm.entry(String::from("Rust")).or_insert(2);

    // char count 
    let mut wc = HashMap::new();

    for c in "racecar".chars() {
        wc.entry(c).and_modify(|x| *x += 1 ).or_insert(1);
    }

    for (k,v) in &wc {
        println!("{k} {v}");
    }

    // word count 
    let mut wc = HashMap::new();

    for w in "happy hours are happy".split_whitespace() {
        
        let count = wc.entry(w).or_insert(0);
        *count += 1;
    }

    for (k,v) in &wc {
        println!("{k} {v}");
    }
}