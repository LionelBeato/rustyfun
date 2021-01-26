/**
 * 
 * Your task is to make a function that can take any non-negative integer as an argument
 *  and return it with its digits in descending order. 
 * Essentially, rearrange the digits to create the highest possible number.
 * 
 */

use std::str;

pub fn descending_order(x: u64) -> u64 {

    
    let string: String = x.to_string();
    let mut char_vec: Vec<char> = string.chars().collect();
    char_vec.sort();
    char_vec.reverse();

    let vec_to_string: String = char_vec.into_iter().collect();
    // let my_string = vec_to_string.to_string();
    let num: u64 = vec_to_string.parse().unwrap();

    num
}

pub fn reverse_order(x: u64) -> u64 {

    // take in x and turn it into a string
    let string: String = x.to_string();

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