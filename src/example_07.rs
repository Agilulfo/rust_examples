pub fn run() {
    let one = String::from("hi");
    let two = String::from("yo");
    let both = create_tuple(&one, &two);

    println!("{} {}", both.0, both.1);

    // let both;
    // {
    //     let one = String::from("hi");
    //     let two = String::from("yo");
    //     both = create_tuple(&one, &two);
    // }
    // println!("{} {}", both.0, both.1);

    // the commented code above would cause the following result.

    // CARGO run
    //    Compiling rust_examples v0.1.0 (/home/corsair/code/rust/rust_examples)
    // error[E0597]: `one` does not live long enough
    //   --> src/example_07.rs:12:29
    //    |
    // 9  |         let one = String::from("hi");
    //    |             --- binding `one` declared here
    // ...
    // 12 |         both = create_tuple(&one, &two);
    //    |                             ^^^^ borrowed value does not live long enough
    // 13 |     }
    //    |     - `one` dropped here while still borrowed
    // 14 |
    // 15 |     println!("{} {}", both.0, both.1);
    //    |                       ------ borrow later used here

    let both;
    let one = String::from("hi");
    {
        let two = String::from("yo");
        both = double_first(&one, &two);
    }
    println!("{} {}", both.0, both.1);
}

// lifetime
// here we have a function that wrap 2 references into a tuple
// the compiler needs to figure how long the valure returned will live.
// its life depends on the input arguments of the function.
// if first or second get deallocated the values withing the tuple
// would stop working
// (it could not depend on variables defined within the function because
// such variables would be cleared at the end of the function)
// Because the compiler cannot always figure out what depend on what,
// it might be necessary to annotate with "'<something>" the dependency between
// inputs and outputs

fn create_tuple<'a, 'b>(first: &'a str, second: &'b str) -> (&'a str, &'b str) {
    (first, second)
}

// here we can tell the compiler that the result will only depend on the first argument
// this allows us to have the result live longer than the second argument
fn double_first<'a>(first: &'a str, second: &str) -> (&'a str, &'a str) {
    println!("{}", second);

    (first, first)
}
