mod main1;

use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("Guess the number");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("couldnt read");

        let guess_int: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let secret_number = thread_rng().gen_range(1..=5);

        //hello_hello();
        println!("You guessed {}", guess);

        println!("The secret number is {}", secret_number);

        match guess_int.cmp(&secret_number) {
            Ordering::Greater => println!("Is Greater"),
            Ordering::Less => println!("Is less"),
            Ordering::Equal => {
                println!("Is equal ... You Won!!!!!");
                break;
            }
        }
    }
}

fn hello_hello() {
    main1::some_function();
}
