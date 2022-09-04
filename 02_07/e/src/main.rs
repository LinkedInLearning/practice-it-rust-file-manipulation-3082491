use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents)
}

fn main() {
    let file_path = "test_file";

    let contents = read_file(&file_path).expect(&format!("unable to read file <{}>", file_path));
    println!("{}", contents);
}
