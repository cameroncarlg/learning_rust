fn return_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn main() {
    let word = String::from("Hello world");
    let first_word = return_first_word(&word[..]);
    println!("{first_word}");
}
