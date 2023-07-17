pub fn run() {
    let x: i8 = 10;

    println!("initially x is: {}", x);
    add_one(x);
    println!("then x is: {}", x);
}

fn add_one(mut item: i8) {
    item = item + 1;
    println!("changin copy of x to: {}", item);
}
