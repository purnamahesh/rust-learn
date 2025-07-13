
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Blue,
    Red
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {

    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {

        user_preference
        .unwrap_or_else(|| self.most_stocked() )

    }

    fn most_stocked(&self) -> ShirtColor {
        let mut red_c = 0;
        let mut blue_c = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => blue_c+=1,
                ShirtColor::Red => red_c+=1,
            }
        };

        if red_c > blue_c { ShirtColor::Red } else { ShirtColor::Blue  }
    }
}

fn main () {
    let inv =  Inventory {
        shirts: vec![ShirtColor::Red, ShirtColor::Blue, ShirtColor::Red]
    };
    let giveaway_shirt = inv.giveaway(None);
    println!("{:?}", giveaway_shirt);

    let giveaway_shirt = inv.giveaway(Some(ShirtColor::Blue));
    println!("{:?}", giveaway_shirt);

    let giveaway_shirt = inv.giveaway(Some(ShirtColor::Red));
    println!("{:?}", giveaway_shirt);
    
}