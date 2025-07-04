use std::io::stdin;
// 0 1 1 2 3 5 8 13 21 34 55 89 
// 1 2 3 4 5 6 7 8  9  10 11 12 
fn fib(n: u32) -> u128 {
    if n == 1 { return 0 }
    else if n == 2 { return 1 }
    let mut a:u128 = 0;
    let mut b:u128 = 1;
    let mut c:u128 = 0;

    for _ in 2..n {
        c = a + b;
        a = b;
        b = c;
    }

    return c
}

fn main() {
    
    let mut inp:String = String::new();

    println!("Enter a number (1-187):");

    stdin()
    .read_line(&mut inp)
    .expect("Failed to read line!");

    let inp:u32 = match inp.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number!");
            return;
        }
    };

    let x = fib(inp);

    println!("fib({inp})={x}");
}