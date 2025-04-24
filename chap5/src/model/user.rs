#[derive(Debug)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl User {

    //associated function
    pub fn print_user_who_is_inactive(&self, status: StatusInactive) {
        println!("user  {:?} is {:?}", self, status);
    }

    //associated function which is not a method of the instance, which are used for construction
    pub fn new() -> User {
        User {
            name: String::from("byorn"),
            email: String::from("<EMAIL>"),
        }
    }
}

pub struct StatusActive;



#[derive(Debug)]
pub struct StatusInactive;


pub struct UserColor(pub u32, pub u32, pub u32);

pub fn new(name: String, email: String) -> User {
    User {
        name,
        email,
    }
}

pub fn print_user(user: &User) {
    println!("user  {:?}", user);
}