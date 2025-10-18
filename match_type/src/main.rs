#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    //let test_quarter1 = value_in_cents(Coin::Quarter(UsState::Alaska));
    //let test_quarter2 = value_in_cents(Coin::Quarter(UsState::Alabama));
    //let test_nickle = value_in_cents(Coin::Nickle);
    //let test_dime = value_in_cents(Coin::Dime);
    //let test_penny = value_in_cents(Coin::Penny);
    //println!("{test_quarter1}");
    //println!("{test_quarter2}");
    //println!("{test_nickle}");
    //println!("{test_dime}");
    //println!("{test_penny}");

    //let five = Some(5);
    //let six = plus_one(five);
    //let none = plus_one(None);

    //let config_max = Some(3u8);
    //if let Some(max) = config_max {
    //    println!("The maximum is configured to be {max}");
    //}
}
