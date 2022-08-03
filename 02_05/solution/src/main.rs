use std::fs;

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect(&format!("unable to read file <{}>", path))
}

fn main() {
    let file_path = "test_file";

    let contents = read_file(file_path);
    println!("{}", contents);
}
