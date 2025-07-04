
fn main () {

    let mut v = Vec::new();

    v.push(10);

    let v = vec![1,2,3,4];

    let first = &v[0];

    let second = v.get(1) ;
    match second {
        Some(_) => (),
        None => {
            println!("IndexOutOfBounds");
            return;
        },
    }
    let second_val = second.unwrap();

    for i in v {
        print!("{} ",i);
    }
    println!();

    let mut mv = vec![1,2,3,4];

    for i in &mut mv {
        *i *= 2;    
        print!("{} ",i);
    }
    println!();

    #[derive(Debug)]
    enum Row {
        Int(i32),
        Text(String),
        Float(f32)
    }

    let mut rows: Vec<Row> = Vec::new();

    rows.push(Row::Int(48));

    rows.push(Row::Text(String::from("Douche")));

    rows.push(Row::Float(10.0));

    for i in rows {
        println!("{:?} ", i );
    }
}