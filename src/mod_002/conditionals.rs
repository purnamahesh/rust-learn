

fn main () {
    let x = 10;
    if x > 5 {
        println!("greater than 5")
    } else if x < 5 {
        println!("less than 5")
    } else {
        println!("is  5")
    }

    // in-line if
    let even_flag:u8 = if x % 2 == 0 {
        println!("1");
        1
    } else {0};

    println!("{even_flag}");
}