pub fn run() {
    let mut sentence = String::from("this sentence was ");

    println!("{}", sentence);
    // differently from what happened it the variable was an integer
    // because the type String does not implement the copy type
    // we would not be able to use again the variable sentence without
    // having the function returning back the String instance
    sentence = append(sentence, "incomplete");
    println!("{}", sentence);
}

// this function take ownership of the string
// and return back the modified string
fn append(mut sentence: String, ending: &str) -> String {
    sentence.push_str(ending);
    println!("{}", sentence);
    sentence
}
