
pub fn add_one(x:i32) -> i32 {
    x + 1
}

//  cargo test -p add_one
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_one() {
        assert_eq!(add_one(10), 11);

        assert_eq!(add_one(-10),-9);
    }     
}