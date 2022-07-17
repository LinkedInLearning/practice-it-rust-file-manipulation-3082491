use std::fs;

fn main() {
    let contents: String = fs::read_to_string("test_file").expect("unable to read file");
    println!("{}", contents);
}
