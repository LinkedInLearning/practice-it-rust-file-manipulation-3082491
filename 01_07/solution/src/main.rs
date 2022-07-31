use std::fs;

fn main() {
    let wanted_string = "a";
    let file_path = "file_with_lines";
    let contents: String = fs::read_to_string(file_path).expect("unable to open file");
    for line in contents.lines() {
        if line.contains(wanted_string) {
            println!("{}", line);
        }
    }
}
