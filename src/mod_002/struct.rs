// struct
// ..syntax -> to copy data from one struct to another
// dbg! trait; #[derive(Debug)]
// Tuple Struct
// Unit-Like Structs
// mehods using impl
//  self
// associated functions ::

struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

#[derive(Debug)]
struct Rectangle {
    length: f32,
    width: f32
}

impl Rectangle {
    fn area(&self) -> f32 { self.width * self.length }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

impl Rectangle {
    fn square(size:f32) -> Self {
        Self {
            length: size, 
            width: size 
        }
    }
}

struct UnitLikeStruct;

struct Color ( u32, u32, u32 );
struct Position ( u32, u32, u32 );

fn main() {

    // example: 1
    
    let user1 = User {
        name: String::from("Mahesh"),
        active: true,
        sign_in_count: 1,
        email: String::from("m@m.com")
    };

    let user2 = User {
        name: String::from("Purna"),
        ..user1
    };

    // println!("{}", user1.email); // error - email moved to user2

    // example 2: Tuple Struct
    let c1 = Color (1, 2, 3);
    let c2 = Color (1, 2, 3);
    let p1 = Position(1, 2, 3);

    let r1 = Rectangle {
        length:10.0,
        width:20.0
    };

    let r2 = Rectangle {
        length:9.0,
        width:10.0
    };

    let a1 = r1.area();

    r1.can_hold(&r2);
    
    let s1 = Rectangle::square(10.0);

    r1.can_hold(&s1);

}