use std::fs;

fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let mut lines = Vec::new();
    let contents = fs::read_to_string(path)?;
    for line in contents.split("\n") {
        lines.push(line.to_string());
    }

    Ok(lines)
}

fn main() {
    let file_path = "file_with_lines";
    let lines: Vec<String> =
        read_file(&file_path).expect(&format!("Error reading file <{}>", &file_path));

    println!("{:#?}", lines);
}
