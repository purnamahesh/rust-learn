
pub mod garden;
pub mod nursery;

use garden::vegetables::Potato;

use crate::nursery::flowers::Rose;

fn main() {

    let _p1 = Potato{weight:30.0, kcal:60.0};

    let _rose1 = Rose {
        color: (34, 56, 43)
    };

}