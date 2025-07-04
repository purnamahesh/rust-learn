

fn main () {

    let mut x = 0;

    loop {  
        print!("{x} ");
        if x == 10 {
            break;
        }
        x+=1;
    }
    println!();

    // loop return
    let mut a = 0;
    let mut sum = 0;
    let _res = loop {
        if a > 10 {
            break sum
        }
        sum += a;
        a += 1;
    };

    // loop with labels

    let mut x = 0;

    'loop_1: loop {
        print!("{x}: ");
        if x == 10 {
            break;
        }
        x+=1;
        let mut y = 0;
        loop {
            print!("{y}");

            if y == 10 {
                break;
            } else if x==8 && y == 7 {
                break 'loop_1;
            }
            y+=1;
        }
        println!();
    }
    
}