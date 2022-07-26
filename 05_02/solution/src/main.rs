use std::collections::HashMap;
use std::fs;

fn read_file(path: &str) -> Result<Vec<Vec<String>>, std::io::Error> {
    let mut result = Vec::new();
    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        let mut words = Vec::new();
        for word in line.split_whitespace() {
            words.push(word.to_string());
        }
        result.push(words);
    }
    Ok(result)
}

fn word_count(lines: &Vec<Vec<String>>) -> HashMap<String, i32> {
    let mut hashmap = HashMap::new();
    for line in lines {
        for word in line {
            let count = hashmap.entry(word.to_string()).or_insert(0);
            *count += 1
        }
    }

    hashmap
}

fn replace_x_with_y_in_place(
    mut lines: Vec<Vec<String>>,
    replace_map: &HashMap<String, String>,
) -> Vec<Vec<String>> {
    for line_index in 0..lines.len() {
        for index in 0..lines[line_index].len() {
            let word = &lines[line_index][index];

            if let Some(value) = replace_map.get(word) {
                lines[line_index][index] = value.to_string();
            }
        }
    }

    lines
}

fn build_sentences(lines: &Vec<Vec<String>>) -> Vec<String> {
    let mut result = Vec::new();
    for line in lines {
        let sentence: String = line.join(" ");
        result.push(sentence);
    }

    result
}

fn write_sentences_to_file(path: &str, sentences: &Vec<String>) -> Result<(), std::io::Error> {
    let text: String = sentences.iter().map(|line| format!("{}\n", line)).collect();
    fs::write(path, text)?;
    Ok(())
}

fn main() {
    let replace_map = HashMap::from([
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

    let content = read_file("alice_chapter_1").unwrap();
    println!("{:?}\n\n", content);

    let content = replace_x_with_y_in_place(content, &replace_map);

    let counter = word_count(&content);

    println!("{:#?}\n\n", counter);

    let sentences = build_sentences(&content);

    println!("{:#?}", sentences);

    let path = "marcus_chapter_1";
    write_sentences_to_file(&path, &sentences).unwrap()
}
