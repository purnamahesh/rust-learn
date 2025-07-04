// use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;
use rand::random_range;

fn main() {
    let mut inp:String = String::new();
    let mut attempt: u32 = 0;
    // let secret_number: u32 = rand::rng().random_range(1..=100);
    let secret_number: u32 = random_range(1..=100);
    let mut guess: u32;

    loop {

        println!("Guess a number between 1 and 100:");

        stdin()
        .read_line(&mut inp)
        .expect("Failed to read line!");

        // guess = inp.trim().parse().expect("Enter a valid number!");

        guess = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        attempt += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!\n"),
            Ordering::Greater => println!("Too Big\n"),
            Ordering::Equal => {
                print!("You Won!");
                break;
            },
        }

        inp.clear();
        
    }

    println!(" {attempt} attempts.");
}