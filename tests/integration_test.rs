
pub mod common;

use rust_learn::demo_tests::Guess;

use crate::common::display;

#[test]
fn test_sub_module() {
    display();
}


#[test]
#[should_panic] // accepts any panic 
pub fn test_guess_panic_ub() {
    Guess::new(101);
}