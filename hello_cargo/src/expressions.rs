//expressions return a value
pub fn my_expression_test() {
    let my_var = {
        let x = 3;
        x + 1 //if i add a semi colone here, it becomes a statement not an expression
    };

    println!("my expression my_var {}", my_var);
}
