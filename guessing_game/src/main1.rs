use std::io;
pub fn some_function(){
    let mut value_to_hold = String::new();
    match io::stdin().read_line(&mut value_to_hold) {
        Ok(_) => println!("The value you entered was {}", value_to_hold),
        Err(_) => println!("Failed to read line")
    }
}
