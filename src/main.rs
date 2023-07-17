// modules and function names are in snake_case
mod example_01;
mod example_02;
mod example_03;
mod example_04;

fn main() {
    // pass a value to a fucion that makes a copy
    // manipulation in fuction do not affect the value
    title(1);
    example_01::run();

    // pass a value by mutable reference so that the changes the function makes
    // affect the value
    title(2);
    example_02::run();

    // function
    title(3);
    example_03::run();

    // function
    title(4);
    example_04::run();
}

fn title(number: u8) {
    println!("--- example #{} ---", number);
}
