#[derive(Debug)]
enum UsState{
    Alaska,
    Alabama,
    Etc,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => {
            5
        }
        Coin::Dime => {
            10
        }
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let penny = Coin::Penny;
    let nick = Coin::Nickel;
    let dime = Coin::Dime;
    let quart = Coin::Quarter(UsState::Alabama);

    println!("Penny value is {}", value_in_cents(penny));
    println!("Nickel value is {}", value_in_cents(nick));
    println!("Dime value is {}", value_in_cents(dime));
    println!("Quarter value is {}", value_in_cents(quart));
}
