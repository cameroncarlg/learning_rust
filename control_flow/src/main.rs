use std::io;

fn main() {
    println!("This is a Fahrenheit/Celsius converter!\n");
    println!("Please enter your number: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("You've entered: {}", input.trim());
}
