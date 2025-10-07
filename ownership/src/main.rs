//fn read(y: bool) {
//    if y {
//        println!("y is true!");
//    }
//}
//
//fn add_suffix(mut n: String) -> String {
//    n.push_str(" Jr.");
//    n
//}

fn main() {
    //let x = true;
    //read(x);

    //let first = String::from("Cameron");
    //let full = add_suffix(first);
    //println!("{full}");
    //println!("{first}");
    //let mut x: Box<i32> = Box::new(1);
    //let a: i32 = *x;
    //println!("a: {a}");
    //println!("x: {x}");
    //*x += 1;
    //println!("a: {a}");
    //println!("x: {x}");

    // Here r1 is of type ReferenceBox<i32>, so we assign it x's reference
    //let r1: &Box<i32> = &x;

    // Here we dereference r1 twice, try taking out one reference.
    // We can see that it is not demanding you assign it a ReferenceBox<i32> type
    // as what is what r1 is. Dereferencing it twice gives us the actual
    // value that x is pointing to, 2
    // r1 -> x -> 2
    //let b: i32 = **r1;
    //println!("r1: {r1}");
    //println!("b: {b}");

    // Here, r2 is of type Reference-i32. We assign it a reference to the
    // dereferenced x, which will be 2.
    //let r2: &i32 = &*x;

    // Then we assign c (of type i32) to a dereferenced r2, which will end up
    // being an actual number, 2
    // Onces a reference, ones an actual number
    //let c: i32 = *r2;
    //println!("r2: {r2}");
    //println!("c: {c}");
    //println!();

    //let x: Box<i32> = Box::new(-1);
    //let x_abs1 = i32::abs(*x);
    //let x_abs2 = x.abs();
    //assert_eq!(x_abs1, x_abs2);

    let mut v: Vec<i32> = vec![0, 1, 2];
    let num: &mut i32 = &mut v[2];
    println!("Third element is {}", *num);

    *num += 1;
    println!("Third element is {}", *num);
    println!("The vector is now {:?}", v);
}
