use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

working_with_vector();
    working_with_strings();
    working_with_hashmap();
}

fn working_with_vector()->Vec<i32>{
    let mut v = Vec::new();

    v.push(1);
    v.push(2);

   for i in &v {
       println!("{}", i);
   }

    // this will panic if the index 1 has not value
   let i2:&i32 = &v[1];
   println!("{}", i2);

    //access any index in the vector like this
    let val:Option<&i32> = v.get(20);

    match val {
        Some(x) => {
            println!("{}", x);
        }
        None => {}
    }


    v.push(3);//mutable borrow
    //println!("{:?}", val); // will not work as am trying to borrow val again when there is already a one mutable borrow
    v
}

fn working_with_strings(){
  //way to init a string
    let s1 = String::from("hello");
   //another way
let mut s2 = "hello".to_string(); //converts a &str to a String

    s2.push_str(" world");
println!("{}", s2);

   //concatenation
    let s3 = s1 + &s2;  //s1 ownershipt goes into s3. s1 has a add method wthich takes in a &str as a parameter. reason s2 &String works because rust can accept a &str as a &str[]
    //Rust uses a deref coercion, which here turns &s2 into &s2[..]

    //another way of concatenation
    let s4 = format!("{} {}", s3, s2);//note i cant now use s1 here because ownership of s1 has been moved to s3
   println!("{}", s4);

    //note String isA String is a wrapper over a Vec<u8>. So s1[0] wont work. it stroes the bytes in the string as a Vec[u8] like golang runes.

    let hello = "Здравствуйте";

    let s = &hello[0..4];

    //s will be Зд.  first4 bytes are the fist two characters.
    for c in "Зд".chars() {//use the chars method instead/
        println!("{c}");
    }

}

fn working_with_hashmap(){
    //Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type.

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("{} scored {}", team_name, score);

    for (key, value) in &scores {//its &scores like in vectors because we dont want to transfer the ownership to the variables key and value
        println!("{key}: {value}");
    }


    //below will move the ownershipt of var3 to the hashmap
    let var3 = String::from("Green");
    scores.insert(var3, 22);

    println!("{scores:?}");
    //println!("{var3:?}");//wont work

    //get the existing value or insert it
let var4 = scores.entry(String::from("Blue")).or_insert(50);
   println!("blue {var4:?}");
 }




