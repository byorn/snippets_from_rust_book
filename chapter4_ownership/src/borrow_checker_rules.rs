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
fn mutable_references(){
    let mut s = String::from("hello");

    //(1)
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM - will fail

    // println!("{}, {}, and {}", r1, r2, r3); -- will fail
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