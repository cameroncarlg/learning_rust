const LYRICS: [&str; 12] = [
    "Twelve drummers drumming",
    "Eleven pipers piping",
    "Ten lords a-leaping",
    "Nine ladies dancing",
    "Eight maids a-milking",
    "Seven swans a-swimming",
    "Six geese a-laying",
    "Five golden rings",
    "Four calling birds",
    "Three French hens",
    "Two turtle doves",
    "A partrige in a pear tree",
];

// Return a string slice that lives for the entire program run.
// &'statuc str -> reference to UTF-8 data baked into the binary
// (no allocation, no lifetime worries)
fn ordinal(n: usize) -> &'static str {
    match n {
        1 => "1st",
        2 => "2nd",
        3 => "3rd",
        4..20 => "th",
        _ => unreachable!(),
    }
}

fn main() {
    // Outer loop: one iteration per day (0..12 -> 1st ... 12th)
    for day in 0..12 {
        println!("On the {} day of Christmas,", ordinal(day + 1));
        println!("my true love gave to me...");

        // Inner loop: build the *reverse* list of gifts for this day.
        // 11 - day ... 11 is the slice we need from the constant array.
        // .rev() makes us start with the highest-numbered gift and walk
        // backwards to index 11-day (inclusive)
        for gift_idx in (11 - day..12).rev() {
            println!("{}.", LYRICS[gift_idx]);
        }
        println!();
    }
}
