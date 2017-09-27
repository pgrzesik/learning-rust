fn main() {
    println!("Value in cents of Penny: {}", value_in_cents(Coin::Penny));
    println!(
        "Value in cents of Quarter: {}",
        value_in_cents(Coin::Quarter(UsState::California)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

#[derive(Debug)]
enum UsState {
    California,
    Georgia,
    Texas
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state: {:?}", state);
            25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}
