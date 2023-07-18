pub fn run() {
    let mut sentence = String::from("this sentence was ");

    // TODO: seems like the exclamation point indicates this is a macro
    // verify what macros are, how to write them.
    // also understand the mechanism that allows to pass an arbitrary number of parameters

    println!("{}", sentence);
    append(&mut sentence, "incomplete");
    println!("{}", sentence);
}

// TODO: understand exact difference between &str and &String
// https://doc.rust-lang.org/stable/book/ch04-03-slices.html#string-literals-as-slices
fn append(sentence: &mut String, ending: &str) {
    sentence.push_str(ending);
    println!("{}", sentence);
}
