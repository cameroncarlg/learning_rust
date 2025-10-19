use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // match on the error. If the file is not found,
            // create it (fc)
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },

            // anything else return this
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    println!("{:?}", greeting_file);
}
