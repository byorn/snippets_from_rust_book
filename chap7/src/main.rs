use crate::gardendd::Asparagus::Chicken;
mod gardendd;

fn main() {
    println!("Hello, world!");
    gardendd::call_me();  
 
    let c = Chicken{name: 1222};
}

//note:
//instead of creating a gardendd.rs file where you export the Asparagus.rs, you can 
//include a mod.rs file inside gardendd that will export the Asparagus.rs with mod Asparagus;