fn main() {
    //const MAX_LOOP = 12;
    let lyrics = [
        "A partrige in a pear tree.",
        "Two turtle doves.",
        "Three French hens.",
        "Four calling birds.",
        "Five golden rings.",
        "Six geese a-laying.",
        "Seven swans a-swimming.",
        "Eight maids a-milking.",
        "Nine ladies dancing.",
        "Ten lords a-leaping.",
        "Eleven pipers piping.",
        "Twelve drummers drumming.",
    ];

    // Rusts range() loops like the following: start...end
    // Outer loops will loop from 0 - 12, noninclusive
    for i in 0..lyrics.len() {
        println!("On the {} day of Christmas,", i + 1);
        println!("my true love gave to me");
        for j in 0..i + 1 {
            println!("{}", lyrics[j]);
        }
        println!();
    }
}
