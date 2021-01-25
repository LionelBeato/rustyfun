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
    println!("{:?}", codewars::desc::descending_order(12345));
}

#[cfg(test)]
pub mod tests {
    
    #[test]
    fn my_test() {
        println!("Hello! I'm in a test!");
    }
}