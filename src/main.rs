mod my_modules;
mod numbers;
mod codewars;

pub fn do_thing() -> u64 {
    4
}

fn main() {
    // my_modules::module_play::print_config();
    // println!("Hello world");
    // println!("Hi");
    // println!("hi world");

    // let my_string = String::from("Howdy");
    let hello = String::from("hello!");

    println!("{}", hello);
    println!("{:?}", numbers::give_value::return_five());
    println!("{:?}", do_thing());
    println!("{:?}", codewars::desc::reverse_order(12345));
    println!("{:?}", codewars::desc::descending_order(45329));

    let x =  |a: i32| -> i32 { a };
    x(2);

    let numbers = vec![1, 2, 3, 4, 5];

    let mapped: Vec<i32> = numbers.iter()
            .map(|number| number * 2)
            .collect();

    // below is the closest thing to reduce or inject
    // this is a sum of all values, returning a single value
    let sum: i32 = mapped.iter()
                    .fold(0, |acc, number| acc + number);


    println!("{:?}", mapped); 
    println!("{}", sum);

    // let x = 3;
    // not allowed! can't capture environment variable
    // fn triple(y: i32) -> i32 { x * y };
    // allowed! closures can capture variables outside its scope
    // let triple = |y: i32| -> i32 { x * y };


    let my_string = "hello world";
    let byte = my_string.as_bytes()[0];
    println!("{}", byte as char); 


}

#[cfg(test)]
pub mod tests {
    
    #[test]
    fn my_test() {
        println!("Hello! I'm in a test!");
    }
}