
pub fn return_five() -> i32 {
    return 5; 
}




#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn five_test() {
        assert_eq!(5, return_five());
    }



}