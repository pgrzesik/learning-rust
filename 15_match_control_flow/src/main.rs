fn main() {
    println!("Value in cents of Penny: {}", value_in_cents(Coin::Penny));
    println!(
        "Value in cents of Quarter: {}",
        value_in_cents(Coin::Quarter(UsState::California)));
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