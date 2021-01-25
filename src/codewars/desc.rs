/**
 * 
 * Your task is to make a function that can take any non-negative integer as an argument
 *  and return it with its digits in descending order. 
 * Essentially, rearrange the digits to create the highest possible number.
 * 
 */


pub fn descending_order(x: u64) -> u64 {
        // take in x and turn it into a string
    
        let string = x.to_string();
    
    
        // invert the string 

        let inverted: String = string
                            .chars()
                            .rev()
                            .collect();
    
        // turn string into number

        let inverted_num: u64 = inverted.parse().unwrap();
    
        // return number

        inverted_num
}
    