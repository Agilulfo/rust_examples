pub fn run() {
    let sentence = String::from("  this   is not a very long sentence ");

    let words = count_words(&sentence);
    println!("counted {} words", words);
}

fn count_words(mut sentence: &str) -> usize {
    let mut count = 0;
    while sentence.len() > 0 {
        let index = find_word(sentence);
        println!("index-----: {}", index);
        sentence = &sentence[index..];
        println!("remain-----: {}", sentence);
        count += 1;
    }
    return count;
}

fn find_word_beginning(sentence: &str) -> usize{
   let mut index = 0;
   for letter in sentence.chars() {
   if letter != ' ' {
   return index;
} else
index += 1;
   }
}
fn find_word(sentence: &str) -> usize {
    let mut index = 0;
    for letter in sentence.chars() {
        println!("letter: {}", letter);
        println!("index: {}", index);

        if letter == ' ' {
            return index + 1;
        }
        index += 1;
    }
    return index;
}
