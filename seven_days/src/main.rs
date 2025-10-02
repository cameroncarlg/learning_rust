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

    let position = ["st", "nd", "rd", "th"];

    // Rusts range() loops like the following: start...end,
    // Outer loops will loop from 0 - 12, noninclusive
    for (i, _line) in lyrics.iter().enumerate() {
        // Check for positional suffixes
        if i == 0 {
            println!("On the {}{} day of Christmas,", i + 1, position[0]);
        } else if i == 1 {
            println!("On the {}{} day of Christmas,", i + 1, position[1]);
        } else if i == 2 {
            println!("On the {}{} day of Christmas,", i + 1, position[2]);
        } else {
            println!("On the {}{} day of Christmas,", i + 1, position[3]);
        }

        //println!("{i}");
        println!("my true love gave to me...");
        for (j, _v) in lyrics.iter().enumerate() {
            println!("{}", lyrics[lyrics.len() - (j + 1)]);
        }
        println!();
    }
}
