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
