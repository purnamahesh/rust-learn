use std::io::stdin;


fn main() {

    let mut opt:String = String::new();
    let mut temp:String = String::new();
    const DEFAULT:u8 = 1;

    println!("\n\n[1] Celcius to Farenheit\n[2] Farenheit to Celcius\n\nChoose an option(default=1):");

    stdin()
    .read_line(&mut opt)
    .expect("Failed at read line!");

    let opt = if opt.trim() == "" { DEFAULT } else {
        match opt.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid Input!");
                return;
            },
        }
    };

    let prompt: &str = if opt == 1 { "Enter Celcius:" } else { "Enter Farenheit:" };

    println!("{prompt}");

    stdin()
    .read_line(&mut temp)
    .expect("Failed at read line!");

    let temp:f32 =  match temp.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid Input!");
            return;
        },
    };

    let result: f32 = if opt == 1 { temp * (9.0/5.0) + 32.0 } else { (temp - 32.0) * (5.0 / 9.0) };

    if opt == 1 { 
        println!("{}C = {}F", temp, result);  
    } else { 
        println!("{}F = {}C", temp, result);
    };

}