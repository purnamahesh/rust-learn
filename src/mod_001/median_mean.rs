use std::{cmp::Ordering, collections::HashMap, i32, io::stdin};



// Given a list of integers, use a vector and return the median (when sorted, the value in
// the middle position) and mode (the value that occurs most often; a hash map will be
// helpful here) of the list.
fn main () {

    let mut inp = String::new();

    stdin()
    .read_line(&mut inp)
    .expect("Failed in read line");

    let mut list:Vec<i32> = Vec::new(); 

    for n in inp.trim().split_whitespace() {
        list.push(
            n.parse::<i32>().expect("Failed to parse")
        );
    }
    inp.clear();
    println!("\n[1] Median\n[2] Mode\n\nEnter operation:");
    stdin()
    .read_line(&mut inp)
    .expect("Failed in read line");

    let opt = inp.trim().parse::<u8>().expect("Failed at parse");

    match opt {
        1 => {
            // Median
            list.sort();
            let n = list.len();
            let mid_idx = ((n as f32) / 2.0) as usize;
            let median = if n % 2 == 0 {
                &list[(mid_idx-1)..=mid_idx]
                .iter().sum()
                // &list[(z-1)..=z]
            } else { 
                &list[mid_idx-1]
            };
            println!("median: {median}");
        },
        2 => {
            // Mode
            let mut hm = HashMap::new();
            // let mut max = u32::MIN;
            // let mut res = u32::MIN;
            for i in &list {
                hm.entry(i).and_modify(|f| {*f += 1}).or_insert(1);
            }

            let res = hm.iter().
            max_by(|x, y| x.1.cmp(&y.1))
            .map(|(x,y)| x);

            println!("hm: {:?}, mode:{}",hm, res.unwrap());
        },
        _ => {
            println!("Invalid Entry");
        }
    }

    
}