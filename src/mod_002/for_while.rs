

fn main () {

    for i in 1..10 {
        println!("5 x {i} = {}", 5*i);
    }

    let arr = [11, 22, 66, 44, 99, 77];
    
    for e in arr {
        print!("{e} ")
    }
    println!();

    let mut a = 10;

    while a > 0 {
        print!("{a}");
        a -= 1;
    }
}