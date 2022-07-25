use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let contents: File = File::open("file_with_lines").unwrap();
    let reader = io::BufReader::new(contents);
    for line in reader.lines() {   
        println!("{}", line.unwrap());
    }
}
