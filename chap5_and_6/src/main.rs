mod model;

use model::user;

fn main() {
  instantiating_structs();
    instantiating_tuple_structs();
  debug_trait_implemented_for_struct();
    method_on_struct();
}

fn instantiating_structs(){
    let u1 = user::User{
        name: String::from("byorn"),
        email: String::from("<EMAIL>"),
    };

    let u2 = user::new("jing".to_string(), "jing@gmail.com".to_string());
    println!("Hello, world!");

    //spread operator like in js
    let u3 = user::User{
        name: String::from("new byorn"),
        ..u1
    };

    println!( "u3 name {} email is: {}", u3.name, u3.email);
}

fn instantiating_tuple_structs(){
    let mut color_green = user::UserColor(23, 23, 23);
    color_green.0 = 12;

    println!("color green is: {}", color_green.0);
}

fn debug_trait_implemented_for_struct(){
    let u1 = user::User{
        name: String::from("byorn"),
        email: String::from("<EMAIL>"),
    };

    user::print_user(&u1);
}


//All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
fn method_on_struct(){
    let u1 = user::User{
        name: String::from("byorn"),
        email: String::from("<EMAIL>"),
    };
let status_inactive = user::StatusInactive;
    u1.print_user_who_is_inactive(status_inactive);

    //calling an associated function as a new construction
let byorn = user::User::new();
    println!("byorn is {:?}", byorn);
}
