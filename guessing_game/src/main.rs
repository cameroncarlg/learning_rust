mod guess;
use guess::Guess;
use rand::Rng;
use std::io::{self, Write};

fn main() {
    let mut rng = rand::thread_rng();
    let rand_num: i32 = rng.gen_range(1..=100);
    //println!("Random gen #: {rand_num}");

    loop {
        print!("Enter a number to guess (1-100): ");
        io::stdout().flush().expect("Failed to flush buffer");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        //println!("Your input is: {input}");
        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a valid number, try again!");
                continue;
            }
        };

        let guess = Guess::new(n);

        match guess {
            n if n.value() > rand_num => println!("Too high! Guess again..."),
            n if n.value() < rand_num => println!("Too low! Guess again..."),
            _ => {
                println!();
                println!("You win!");
                break;
            }
        }
    }
}
