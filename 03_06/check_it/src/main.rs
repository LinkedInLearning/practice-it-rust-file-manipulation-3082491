fn get_words(text: &str) -> Vec<String> {
    let mut words = Vec::new();
    for word in text.split_whitespace() {
        words.push(word.to_string());
    }
    words
}

fn main() {
    let contents = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    let words = get_words(&contents);

    println!("{:?}", words);
}
