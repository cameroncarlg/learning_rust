use std::{
    fs,
    //fs::File,
    //io::{Error, ErrorKind, Read},
    io::Error,
};

fn read_username_from_file() -> Result<String, Error> {
    // if the value from Result is Ok -> place it in to variable
    // "?" allows us to have syntactic sugar over Result<T,E>

    //let mut username = String::new();

    //File::open("hello.txt")?.read_to_string(&mut username)?;

    //ok(username)

    // even shorter...
    fs::read_to_string("hello.txt")
}

fn main() {
    let username = read_username_from_file();
    println!("{}", username.unwrap().trim());
    //println!("{}", username.unwrap().len());

    //let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //    if error.kind() == ErrorKind::NotFound {

    //        // 'unwrap_or_else' just mean execute the create file method
    //        // unless theres an error, if an error, panic
    //        File::create("hello.txt").unwrap_or_else(|error| {
    //            panic!("Problem creating the file: {error:?}");
    //        })
    //    } else {
    //        panic!("Problem opening the file: {error:?}");
    //    }
    //});

    // here we can spit out text for the error
    //let greeting_file =
    //    File::open("hello.txt").expect("hello.txt should be included in this project");

    //println!("{:?}", greeting_file);
}
