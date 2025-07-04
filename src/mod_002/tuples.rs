fn main() {
    // tuple: fixed length, heterogenous

    let tup = ("mahesh", 1, 2.5, 'm');

    let (name, _rank, _rating, _sex) = tup; // destructuring

    println!("Name: {name}, Rank: {}, Rating: {}", tup.1, tup.2);

    let _tup2 = (); // empty tuple is called `unit`

    let mut tup = ("mahesh", 1, 2.5, 'm');
    tup.2 = 3.0;

    println!("Name: {name}, Rank: {}, Rating: {}", tup.1, tup.2);

}