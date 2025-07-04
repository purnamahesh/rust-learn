
use std::cmp::PartialOrd;

fn largest<T: PartialOrd>(list:&[T]) -> &T {
    let mut max = &list[0];
    for val in list {
        if val > max {
            max  = val;
        }
    };
    max
} 

#[derive(Debug)]
struct Point<T> { x:T, y:T }

impl Point<f32> { // implementation based on specific type
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }

    fn y(&self) -> &T { &self.y }
}

mod generics{
    #[derive(Debug)]
    pub struct Point<T, U> { pub x:T, pub y:U }

    impl<T, U> Point<T, U>  {
        fn x(&self) -> &T { &self.x }
    
        fn y(&self) -> &U { &self.y }

        fn mixup<T2, U2>(self, other: Point<T2, U2>) -> Point<T, U2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }
}

// enum Result<T, E> {
//     OK(T),
//     Err(E)
// }

fn main() {
    let l = vec![3,64,2,5,35,2,53,6,23];

    let larg = largest(&l);

    println!("{}", larg);

    let p1 = Point { x: 10, y: 20 };
    let p2 = generics::Point { x: 10.0, y: 20 };

    println!("{:?}\n{:?}", p1, p2);
}