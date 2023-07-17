pub fn run() {
    let x: i8 = 10;

    println!("initially x is: {}", x);
    // here x is copied when passed to the function
    add_one(x);
    // so even if the function modifies the value the original remains the same
    println!("then x is: {}", x);
}

// the argument definition is diveded in two parts by ':'
// mut item in this case tells that the variable will be
// changed inside the function. it is kinda like writing "let mut item"
// "i8" specify the type
fn add_one(mut item: i8) {
    item = item + 1;
    println!("changing copy of x to: {}", item);
}
