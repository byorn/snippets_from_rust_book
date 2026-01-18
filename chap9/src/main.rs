use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};

fn main() {
    println!("Hello, world!");

recoverable_error();
}

fn recoverable_error() {
    /*
    enum Result<T, E> {
    Ok(T),
    Err(E),
     */

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };


    /** another way **/
    let greeting_file1 = File::open("hello.txt").unwrap();

    /** another way **/ /** more idiomatic **/
    let greeting_file2   = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

fn unrecoverable_error() {
    //unrecoverable error
    panic!("fails");
    //$ RUST_BACKTRACE=1 cargo run to get the stacktrace
}

fn propogating_error() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//A Shortcut for Propagating Errors: the ? Operator

/*There is a difference between what the match expression from Listing 9-6 does and what the ? operator does: error values
that have the ? operator called on them go through the from function,
defined in the From trait in the standard library,
which is used to convert values from one type into another.
When the ? operator calls the from function, the error type received is converted
into the error type defined in the return type of the current function.

This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
 */

/*For example, we could change the read_username_from_file function in Listing 9-7 to return a custom error type named OurError that we define.
If we also define impl From<io::Error> for OurError to construct an instance of OurError from an io::Error,
then the ? operator calls in the body of read_username_from_file will call
from and convert the error types without needing to add any more code to the function.
 */
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
/*
Where The ? Operator Can Be Used
The ? operator can only be used in functions whose return type is compatible with the value the ?
 is used on. This is because the ? operator is defined to perform an early return of a value out
 of the function, in the same manner as the match expression we defined in Listing 9-6.
 In Listing 9-6, the match was using a Result value, and the early return arm returned an Err(e) value. The return type of the function has to be a Result so that
it’s compatible with this return
 */

//Return type can be:
// ? operator in a function that returns Result, Option, or another type that implements FromResidual.