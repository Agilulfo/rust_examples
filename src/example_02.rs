pub fn run() {
    let mut x = 10;

    println!("initially x is: {}", x);
    // here we are passing a mutable reference
    // this means that changes made by the function will remain
    // after the function call
    add_one(&mut x);
    println!("then x is: {}", x);
}

// the type of the function argument is &mut mutable reference
// notice we do not write "mut item: &mut i8" because
// the reference cannot be changed.
fn add_one(item: &mut i8) {
    // as far as I understand * is the dereferencing operator
    // here we need to dereference the reference if
    // we want to access his value
    *item = *item + 1;

    // why don't we need to dereference in here?
    println!("changin copy of x to: {}", item);
}
