// use rand::Rng;
use std::io::stdin;
use std::cmp::Ordering;
use rand::random_range;

struct Guess { value:u8 }

impl Guess {
    fn new(value: u8) -> Self {
        if value < 1 || value > 100 {
            panic!("Enter a value between 1 and 100");
        }
        Guess { value: value }
    }
}

fn main() {
    let mut inp:String = String::new();
    let mut attempt: u32 = 0;
    let secret_number: u8 = random_range(1..=100);
    let mut guess: Guess;

    loop {

        println!("Guess a number between 1 and 100:");

        stdin()
        .read_line(&mut inp)
        .expect("Failed to read line!");

        // guess = inp.trim().parse().expect("Enter a valid number!");

        guess = match inp.trim().parse() {
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        attempt += 1;

        match guess.value.cmp(&secret_number) {
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