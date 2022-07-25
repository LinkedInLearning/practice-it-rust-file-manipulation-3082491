use std::fs;

fn main() {
    let wanted_string = "a";
    let contents: String = fs::read_to_string("file_with_lines").expect("unable to opne file");
    for line in contents.lines(){
        if line.contains(wanted_string){
            println!("{}", line);
        }
    }
}
