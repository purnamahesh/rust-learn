
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_by__size() {
        let shoes = vec![
            Shoe { size:10, style: String::from("sneaker") },
            Shoe { size:13, style: String::from("sandal") },
            Shoe { size:10, style: String::from("boot") },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe { size:10, style: String::from("sneaker") },
                Shoe { size:10, style: String::from("boot") },
            ] 
        );
    }
}

fn main () {

    let x = vec![1,2,3];
    
    let x_iter = x.iter();

    for e in x {
        println!("Get: {}", e);
    }

    let y = vec![String::from("1"), String::from("2")];

    // into iter
    let into_iter = y.iter(); 

    let into_iter = y.into_iter();

    // iteraator adaptors -> iterators that retuen other iterators
    // eg: map

    let z = vec![1,2,3];    

    let a:Vec<i32> = z
    .iter()
    .map(|x| x + 1)
    .collect();


}