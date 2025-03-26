mod ownership_and_functions;
mod return_values_and_scope;
mod borrow_checker_rules;

fn main() {
    println!("Hello, world!");
}

fn ownership1(){
    //(1)
    let s = String::from("hello");
    let s1 = s;

    //below line won't compile, cause value has been moved to s1
    //println!("s = {}", s)

    //means the s pointer (the address and lenght) properties
    //have been moved to s1

    //(2)
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
    //types such as integers float etc have fixed sizes
    //and these values are directly kept in the stack.
    //they use the Copy trait underneath. values get trivially copied and not moved.
}

fn ownership2(){
    ownership_and_functions::ownership_in_functions();
}

fn ownership3(){
    return_values_and_scope::return_values_and_scope();
}

