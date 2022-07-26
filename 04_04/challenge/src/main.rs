fn main() {
    let content = String::from("This is the first line\nThe second line is a little longer\nLine 3 is short\nThe 4th line is the first non-prime\nThe 5th line has the starting five");
    let replace_map = HashMap::from([
        ("first".to_string(), "last".to_string()),
        ("line".to_string(), "entry".to_string()),
    ]);

    // Implement the get_words and replace_x_with_y_in_place functions
    //let words = get_words(&content);
    //let new_words = replace_x_with_y_in_place(words.clone(), &replace_map);

    //println!("{:?}", new_words);
}
