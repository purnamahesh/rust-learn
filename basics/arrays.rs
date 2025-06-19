use std::io::stdin;

fn main() {
    // array: fixed length, homogenous, memory allocated at stack

    let mut arr = ["mahesh", "1", "2.5", "m"];

    arr[2] = "5";

    println!("Name: {}, Rank: {}, Rating: {}", arr[0], arr[1], arr[2]);

    let _arr2: [char; 3] = ['p', 'q', 'r'];

    let arr3 = [3; 5]; // multiply

    for i in arr3 {
        print!("{} ", i.to_string());
    }

    let arr = [35, 56, 23, 64, 76, 12, 76];

    let mut inp: String = String::new();

    stdin().read_line(&mut inp).expect("Failed to read line");

    let indx: usize = match inp.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Enter a valid number");
            return;
        }
    };

    println!("Element at {} is {}", indx, arr[indx]);
}
