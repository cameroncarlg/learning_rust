use std::slice::Iter;

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v = vec![1, 2, 3];
    let mut iter: Iter<'_, i32> = v.iter();

    let pos1 = iter.next().unwrap();
    let pos2 = iter.next().unwrap();
    let pos3 = iter.next().unwrap();
    let end = iter.next();
    println!("{pos1}, {pos2}, {pos3}, {:?}", end);

    let big_v = vec![
        SpreadSheetCell::Int(4),
        SpreadSheetCell::Float(3.2),
        SpreadSheetCell::Text(String::from("ASDF")),
    ];

    println!("{:?}", big_v);
    println!("{:?}", v);
}
