mod enums;
use enums::*;
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddrKind1 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}



fn main() {

    //using enums. attaching data to an enum
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));

    //can have different types attached to the enum
    let home = IpAddrKind1::V4(127, 0, 0, 1);
    let loopback = IpAddrKind1::V6(String::from("::1"));

    //enums can have methods
    let m = Message::Write(String::from("hello"));
    m.call();


    //calling an enum
    let five = enums::Coin::Quarter(enums::UsState::Alabama);
    let _ten = enums::Coin::Dime;
    let _one_hundred = enums::Coin::Penny;

    println!("Value in cents: {}", value_in_cents(five));

    //handling option for nulls
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    //if let, works same way as match, but focus only on the Some and not the _
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
    /* is same as
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
     */

    /***
    If you have a situation in which your program has logic that is too verbose to express using a match, remember that
    if let and let else
    are in your Rust toolbox as well.

    */
}

//handling enum types with match expression
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

//remember to handle all possible cases for Option in the match expression.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn handle_all_cases(dice_roll: i32)  {
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        //or
        //other => somefunction(other)
    }


}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}