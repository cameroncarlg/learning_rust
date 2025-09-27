use std::io;

fn temp_conv(s: String) -> String {
    const NUM_32: f32 = 32.0;
    if s.len() < 0 || s.len() < 3 {
        return "Your number is not valid, run the program again.".to_string();
    } else {
        //println!("{}", s.len());
        //s.split_at()
        let (temp, fc) = s.trim().split_at(2);
        println!("Temp: {temp}");
        println!("Unit: {fc}");

        if fc == "F" {
            let temp1: f32 = temp.parse().unwrap();
            println!("Its Fahrenheit!");
            println!("Cel?{}", ((temp1 - NUM_32) * (5.0 / 9.0)));
        } else if fc == "C" {
            let temp1: f32 = temp.parse().unwrap();
            println!("It's fucking Celcius!");
            println!("Far?{}", ((temp1 * (9.0 / 5.0)) + NUM_32))
        } else {
            println!("You entered an invalid temperature unit.");
        }
        s
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    //println!("Here's your input len: {}", input.trim().len());
    let parsed_string = temp_conv(input);
    println!("Here is the string parsed: {parsed_string}");

    //let temp1 = String::from("45F");
    //let temp2 = String::from("32C");
    //println!("{:?}", temp1.len());
    //println!("{:?}", temp2.trim().len());
}
