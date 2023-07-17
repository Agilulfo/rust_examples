pub fn run() {
    let mut quote = String::from("this was ");

    println!("{}", quote);
    append(&mut quote, "incomplete");
    println!("{}", quote);
}

fn append(sentence: &mut String, ending: &str) {
    sentence.push_str(ending);
    println!("{}", sentence);
}
