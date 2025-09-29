use std::io::{self, Write};

fn fib(x: i32) -> i32 {
    if x == 0 {
        return x;
    }
    if x == 1 {
        return x;
    }
    fib(x - 1) + fib(x - 2)
}

fn main() {
    print!("Enter a number to generate the nth Fibonacci sequence: ");
    io::stdout().flush().expect("Failed to flush buffer");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let n: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(e) => {
            println!("Invalid input: {}", e);
            return;
        }
    };

    let result = fib(n);
    println!("Here is your nth Fib number: {result}");
}
