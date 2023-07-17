pub fn run() {
    let mut x: i8 = 10;

    println!("initially x is: {}", x);
    add_one(&mut x);
    println!("then x is: {}", x);
}

fn add_one(item: &mut i8) {
    *item = *item + 1;
    println!("changin copy of x to: {}", item);
}
