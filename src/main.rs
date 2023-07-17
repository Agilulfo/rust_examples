mod example_1;
mod example_2;
mod example_3;

fn main() {
    // pass a value to a fucion that makes a copy
    // manipulation in fuction do not affect the value
    title(1);
    example_1::run();

    // pass a value by reference so that the changed the function makes
    // affect the value
    title(2);
    example_2::run();

    title(3);
    example_3::run();
}

fn title(number: u8) {
   println!("--- example #{} ---", number);
}
