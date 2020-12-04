// import for standard input/output
use std::io; 
use rand::Rng; 
use std::cmp::Ordering;



fn print_thing() {
    println!("Thing"); 
}

// String type is reserved for mutability and ownership, prefer &str for parameters! 
// https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html
fn param_print(x:&str) {
    println!("{}", x); 
}
fn guess() {

    let wow = "wow";
    

    param_print(wow);
    print_thing(); 

    println!("Hello, world!");

    println!("Please input your guess.");
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y); 


    loop {
        println!("Guess the number!"); 

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); 

        let guess: u32 = match guess
                               .trim()
                               .parse() {
                            Ok(num) => num,
                            Err(_) => {
                                    println!("not a valid input!"); 
                                    continue
                            }
                        };

        let secret_number = rand::thread_rng()
                                  .gen_range(1, 10); 

        println!("The  secret number is: {}", secret_number); 

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
