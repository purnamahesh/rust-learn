
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main () {

    

    let _some_number = Some(5);
    let _some_char = Some('5');

    let mut absent_number: Option<i32> = None;

    absent_number = Some(32);

    match absent_number {
        Option::None => println!("None"),
        Option::Some(a) => {
            println!("{}", a);
        }
    };

    let a = 10;

    let _b:u32 = match a {
        1 => 1,
        5 => 2,
        other => other + 1,
    };

    let _b:u32 = match a {
        1 => 1,
        5 => 2,
        _ => 10, // if don't want to use var use _
    };

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
    fn move_player(_a: u8) {}

    let dice_roll:u8 = 6;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // if let
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("THe maxium is configured to be {}", max),
        None => (),
    }

    println!("{:?}", config_max);

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    println!("{:?}", config_max);

    

}