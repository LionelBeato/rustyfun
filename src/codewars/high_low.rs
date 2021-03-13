/**
 * 
 * In this little assignment you are given 
 * a string of space separated numbers, 
 * and have to return the highest and lowest number.
 * 
 * All numbers are valid Int32, no need to validate them.
 * There will always be at least one number in the input string.
 * Output string must be two numbers separated by a single space, 
 * and highest number is first. 
 * 
*/

fn high_and_low(numbers: &str) -> String {

    let mut result = numbers.split_whitespace()
            // .map(|x:&str| x.parse::<i32>().unwrap())
            .flat_map(str::parse)
            .collect::<Vec<i32>>();

    result.sort();
    result.iter().for_each(|x| println!("{}", x));

    let highest:String = result.last().unwrap().to_string();
    let lowest:String = result.get(0).unwrap().to_string();

    // highest + " " + &lowest;
    format!("{} {}", highest, lowest)
  }

  #[test]
  fn high_and_low_test() {
    assert_eq!("42 -9", high_and_low("8 3 -5 42 -1 0 0 -9 4 7 4 -4"));
  }
