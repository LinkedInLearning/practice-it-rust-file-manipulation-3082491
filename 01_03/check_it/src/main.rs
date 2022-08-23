use std::fs;

fn main() {
    let file_path = "test_file";
    let contents: String = fs::read_to_string(file_path).expect("unable to read file");
    println!("{}", contents);
}
