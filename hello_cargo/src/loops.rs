pub fn test_loop_statement() {
    let mut y = 1;
    loop {
        y += 1;
        if y == 10 {
            println!("y reached 10");
            break;
        }
    }
}

pub fn iterate_an_array() {
    let arr = [1, 2, 3, 4, 5];
    for number in arr.iter() {
        println!("iterate an array {}", number);
    }

    for element in arr {
        println!("iterate an array with for in {} ", element);
    }
}

pub fn with_while() {
    let x = [1, 2, 3];
    let mut i = 0;
    while i < x.len() {
        println!("with while {}", x[i]);
        i = i + 1;
    }
}
