use std::io::stdin;

// Convert strings to pig latin. The first consonant of each word is moved to the end of the
// word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have
// “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details
// about UTF-8 encoding!
fn main () {

    let mut inp = String::new();

    stdin()
    .read_line(&mut inp)
    .expect("Failed at read line");

    let x = inp.trim().to_lowercase().chars().nth(0).unwrap();

    let ord = (x as u8) - 96;
    let vowels = vec![1,5,9,15,21];

    inp = if vowels.iter().any(|c| *c == ord) { inp.trim().to_owned() + "-hay" } 
            else { inp.pop().unwrap().to_string() + &inp + &"-ay" };

    println!("{inp}");
}