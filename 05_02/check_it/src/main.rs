use std::collections::HashMap;
use std::fs;

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let contents = fs::read_to_string(path)?;

    let lines: Vec<Vec<String>> = contents
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.to_string())
                .collect()
        })
        .collect();
    Ok(lines)
}

fn word_count(lines: &Vec<Vec<String>>) -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();
    for line in lines {
        for word in line {
            hashmap
                .entry(word.to_string())
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    hashmap
}

fn replace_x_with_y_in_place(
    mut lines: Vec<Vec<String>>,
    replace_map: &HashMap<String, String>,
) -> Vec<Vec<String>> {
    for line in lines.iter_mut() {
        for word in line.iter_mut() {
            if let Some(new_word) = replace_map.get(word) {
                *word = new_word.to_string()
            }
        }
    }
    lines
}

fn build_lines(old_lines: &Vec<Vec<String>>) -> Vec<String> {
    let mut new_lines = Vec::new();
    for line in old_lines {
        let sentences: String = line.join(" ");
        new_lines.push(sentences);
    }
    new_lines
}

fn write_lines_to_file(path: &str, lines: &Vec<String>) -> Result<(), std::io::Error> {
    let text: String = lines.join("\n");
    fs::write(path, text)?;
    Ok(())
}

fn main() {
    let replacement_map = HashMap::from([
        ("herself".to_string(), "himself".to_string()),
        ("herself,".to_string(), "himself,".to_string()),
        ("her.".to_string(), "him.".to_string()),
        ("she".to_string(), "he".to_string()),
        ("(she".to_string(), "(he".to_string()),
        ("her".to_string(), "his".to_string()),
        ("Alice's".to_string(), "Marcus's".to_string()),
        ("Alice!".to_string(), "Marcus!".to_string()),
        ("Alice,".to_string(), "Marcus,".to_string()),
        ("Alice;".to_string(), "Marcus;".to_string()),
        ("She".to_string(), "He".to_string()),
        ("(Alice".to_string(), "(Marcus".to_string()),
        ("Alice,)".to_string(), "Marcus,)".to_string()),
        ("she'll".to_string(), "he'll".to_string()),
        ("she’ll".to_string(), "he’ll".to_string()),
        ("Alice".to_string(), "Marcus".to_string()),
        ("her,".to_string(), "him,".to_string()),
        ("Alice’s".to_string(), "Marcus’s".to_string()),
        ("girl".to_string(), "boy".to_string()),
    ]);

    // Format data
    let file_path = "alice_chapter_1";
    let lines = read_file(file_path).expect(&format!("Error reading file <{}>", &file_path));

    // Manipulate data
    let new_lines = replace_x_with_y_in_place(lines, &replacement_map);

    // Used for Discovery
    let counter = word_count(&new_lines);
    println!("{:#?}\n\n", counter);

    // Reconstruct data
    let vec_of_lines = build_lines(&new_lines);

    // Write data to file
    let path = "marcus_chapter_1";
    write_lines_to_file(path, &vec_of_lines).unwrap()
}
