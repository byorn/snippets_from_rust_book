#[derive(Debug)] // so we can inspect the state in a minute
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}