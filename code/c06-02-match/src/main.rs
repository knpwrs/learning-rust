#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // snip
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quater from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn u8_value(i: u8) {
    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("some other number"),
        // _ => (), // Do nothing, `()` is the unit value
    }
}

fn some_u8_value(i: Option<u8>) {
    match i {
        Some(3) => println!("explicitly three"),
        _ => println!("must be none"),
    }
}

fn main() {
    let coin = Coin::Quarter(UsState::Alaska);
    println!("{} cent(s)", value_in_cents(&coin));
    let some_number = Some(5);
    let one_more = plus_one(some_number);
    println!("{:?}", one_more);
    let no_number = None;
    let another = plus_one(no_number);
    println!("{:?}", another);
    u8_value(1);
    u8_value(3);
    u8_value(5);
    some_u8_value(Some(3));
    some_u8_value(None);
}
