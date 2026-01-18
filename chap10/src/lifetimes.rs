//lifetimes ensure that references are valid as long as we need them to be.

//reference in Rust has a lifetime, which is the scope for which that reference is valid.

/*
fn will_fail_compilation() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+

*/


//JUST READ THIS . ITS UNDERSTANDABLE BIT TOO MUCH though
//https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

pub fn test_lifetime(){
   println!("ok {}", longest("abc","defg")) ;
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


/*
 *
 * “Do I always need to write lifetimes?”

 No. Rust uses lifetime elision rules that cover common cases.

 You typically only write lifetimes when:
	•	returning references
	•	structs storing references
	•	complex trait bounds / iterators / async
 *
 *
 *
 *
 *
 * A super practical way to think about 'a

 When you see 'a, read it as:

 “This reference is valid for at least some scope 'a.”

 And when you see something like:
	•	input refs have 'a
	•	output ref has 'a
 it means:

 “The output reference is tied to the same borrowed data lifetime.”

 It’s not “how long in seconds”. It’s “which scope it belongs to”.





 struct User<'a> {
     name: &'a str,
 }

 Read this as:

 “User cannot live longer than 'a, the lifetime of the string it borrows.”



 */
