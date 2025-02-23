mod expressions;
mod functions;
mod loops;

fn main() {
    //mutuable
    let mut x = "abc";
    x = "def";
    println!("value of x {}", x);

    //consts
    const Y_CONST: u32 = 233;
    println!("value of y {}", Y_CONST);

    //shadowing
    let spaces = "   ";
    let spaces = spaces.len();

    println!("value of spaces {}", spaces);

    //accessing tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tup value {}{}{}", x, y, z);
    let first_value_in_tup = tup.0;
    println!("first value in tup {}", first_value_in_tup);

    let dbl: (char, i8) = ('x', 12);
    println!("dbl value is {}", dbl.0);

    //accessing arrays and avoid index of bounds.
    let my_array = [10, 30, 40];
    let correct_val_handled = match my_array.get(5) {
        Some(&val) => val,
        None => my_array[1],
    };

    println!("Correct val handles {}", correct_val_handled);

    let cleaner_way_getting_iobounds_var = my_array.get(5).unwrap_or(&10);

    println!(
        "cleaner_way_getting_iobounds_var {}",
        cleaner_way_getting_iobounds_var
    );

    //testing functions
    functions::test_funcs("hello world");

    // a statement doesnt return a value an expression does.
    expressions::my_expression_test();

    //function returns the last value
    println!("print fun return val {}", five());

    //if expressions
    let number = 15;

    if number > 10 {
        println!("Greater than 10");
    }

    let my_new_number = if number > 10 { 20 } else { 9 };
    println!("if expression {}", my_new_number);

    //testing loops
    loops::iterate_an_array();
    loops::with_while();
    loops::test_loop_statement();
}

fn five() -> i32 {
    5
}
