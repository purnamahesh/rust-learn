// #[test]
// assert, assert_eq, assert_nq
// custom error message
// should_panic
//should panic with expected
// return Result
// default tests run in parallel
// cargo test -- --test-threads=1 --show-output (print for success testcases)
// cargo test it_works
// [ignore] , --ignored , and --include-ignored
// #[cfg(test)] module for each code file - for unit testing
// testing private functions using 'use super::*'


pub struct Guess { _value: u8 }

impl Guess {
    pub fn new(value:u8) -> Self {
        if value < 1 || value > 100 {
            panic!("Value should be between 1 and 100, but got {}", value)
        }

        Guess { _value: value }
    } 
}

// use super::Guess;

#[test]
pub fn it_works() {
    let x = 2 + 2;
    assert!(x == 4,"Expecting 4 got {}",x);
}

#[test]
#[should_panic(expected="Expecting 4")]
pub fn it_dont_work() {
    let x = 3 + 2;
    assert!(x == 4,"Expecting 4 got {}",x);
}

#[test]
#[should_panic] // accepts any panic 
pub fn test_guess_panic_ub() {
    Guess::new(101);
}

#[test]
#[should_panic(expected="Value should be between ")]
pub fn test_guess_panic_lb() {
    Guess::new(0);
}

#[test]
#[ignore = "no reason"]
pub fn test_guess_lb() {
    Guess::new(1);
}

#[test]
pub fn test_guess_ub() {
    Guess::new(100);
}

#[test]
pub fn return_result() -> Result<(), String>{
    // this way we can use ? syntax
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("error"))
    }
}

