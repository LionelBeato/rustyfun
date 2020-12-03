// import for standard input/output
use std::io; 
use rand::Rng; 
use std::cmp::Ordering;

fn main() {
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
