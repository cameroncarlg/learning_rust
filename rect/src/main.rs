#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    let mut r = Rectangle {
        width: 2,
        height: 8,
    };

    let area1 = r.area();
    let area2 = Rectangle::area(&r);

    println!("Here is the area for area1: {}", area1);
    println!("Here is the area for area2: {}", area2);
    assert_eq!(area1, area2);

    r.set_width(2);
    Rectangle::set_width(&mut r, 2);

    //println!(
    //    "The area of the rectangle is {} square pixels.",
    //    rect1.area()
    //);
    //println!(
    //    "It is {} that the width of the rectangle is greater than 5",
    //    rect1.width()
    //);
    //
    //println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));

    //println!("Can rect2 hold rect3: {}", rect2.can_hold(&rect3));
}
