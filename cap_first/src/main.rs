//fn cap_first(v: &mut Vec<char>) {
//    let c = &v[0];
//    if c.is_ascii_lowercase() {
//        let up = c.to_ascii_uppercase();
//        v[0] = up
//    } else {
//        println!("No need!")
//    }
//}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Sir Marcob"));
    let full = name_clone.join(" ");
    full
}

fn main() {
    //let mut lis: Vec<char> = vec!['A', 'b', 'c'];
    //println!("{:?}", lis);
    //cap_first(&mut lis);
    //println!("{:?}", lis);

    let first_name: Vec<String> = vec![String::from("Cameron"), String::from("asdf")];
    let full_name = stringify_name_with_title(&first_name);
    println!("{:?}", full_name);
}
