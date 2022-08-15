use std::fs;

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut lines = Vec::new();
    let contents = fs::read_to_string(path)?;

    for line in contents.lines() {
        let mut words = Vec::new();

        for word in line.split_whitespace() {
            words.push(word.to_string());
        }

        lines.push(words);
    }
    Ok(lines)
}

fn main() {
    let file_path = "file_with_lines";

    let lines = read_file(file_path).expect(&format!("Unable to read file <{}>", &file_path));

    println!("{:?}", lines);
}
