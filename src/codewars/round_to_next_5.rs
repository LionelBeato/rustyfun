
/**
 * 
 * Given an integer as input, 
 * can you round it to the next (meaning, "higher") multiple of 5?
 * 
 */
fn round_to_next_5(n: i32) -> i32 {
    // take n
    // round up n to next 5
    // return rounded up value 

    // completely rotted my brain finding this solution
    // somehow I didn't think to use simple imperative code 
    let mut x = n;
    while x % 5 != 0 {
        x += 1;
    }
    x
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let actual = round_to_next_5(1);
        println!("{}", actual);
        assert_eq!(actual, 5);
    }
    #[test]
    fn test_basic_neg() {
        assert_eq!(round_to_next_5(-1), 0);
    }
    #[test]
    fn test_extended() {
        let tests = [[0,0],[1,5],[-1,0],[-5,-5],[3,5],[5,5],[7,10],[20,20],[39,40],[990,990],[121,125],[555,555]];
        for [x, out] in tests.iter() {
            assert_eq!(round_to_next_5(*x), *out);
        }
    }
}
