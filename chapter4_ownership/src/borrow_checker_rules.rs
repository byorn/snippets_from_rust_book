fn reference1(){
    let s = String::from("hello");
    let len = get_length(&s);

    //note ownership is not transfered.
    println!("The length of '{}' is {}.", s, len);
}

//this is also called borrowing
fn get_length(s:  &String) -> usize {
    s.len()
}


//borrow checker -- also type in chatgpt borrow checker rules. pretty clear.!

//This is the rule perfectly understood by me!
//if you have a mutable reference it is only allowed once! at the point of reading it
//you can have any amount of immutable references
pub fn mutable_references(){
    let mut s = String::from("hello");

    //(1)

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s;

    // println!("{}, {}, and {}", r1, r2, r3); -- will fail
    // println!("{}", r1); -- will fail
    println!("r3{}", r3); //will succeed
    //println("{}", r1); -- will fail
}

fn multiple_mutable_references_ok(){
    let mut s = String::from("hello");
    let r1 = &mut s;

    println!("{}", r1);
    let r2 = &mut s;

    println!("{}", r2);


    //will fail --->  println!("{}", r1);
}

fn multiple_mutable_references_fail() {
    let mut s = String::from("hello");
    let r1 = &mut s;

    let r2 = &mut s;

    //will fail:  println!("{} {}", r1, r2);
    //will fail:  println!("{}", r1);
    println!("{}", r2);
}
fn mutable_references_used_later(){
    let mut s = String::from("hello");


    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{}, {}", r1, r2);

    let r3 = &mut s;//will not fail!

    println!("{}", r3);


}

/*
uncomment to understand


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!:w!
*/

// A slice is a kind of reference, so it does not have ownership.
pub fn the_slice_type(){
    let s = String::from("slice");

    let len = s.len();

    let slice1 = &s[0..len];
    let slice2 = &s[..];
    println!("slice1 = {}, slice2 = {}", slice1, slice2);
}

fn slices_passed_into_functions(){
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
