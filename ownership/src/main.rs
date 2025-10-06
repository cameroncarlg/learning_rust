fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}

fn add_suffix(mut n: String) -> String {
    n.push_str(" Jr.");
    n
}

fn main() {
    let x = true;
    read(x);

    let first = String::from("Cameron");
    let full = add_suffix(first);
    println!("{full}");
    //println!("{first}");
}
