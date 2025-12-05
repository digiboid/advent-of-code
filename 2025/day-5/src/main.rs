use std::fs;
fn parse(path: &str) {
    let contents = fs::read_to_string(path).expect("Where's the file nanoda?");
    let contents: Vec<&str> = contents.lines().collect();
    dbg!(contents);
}

fn main() {
    let contents = parse("test_input.txt");
    println!("Hello, world!");
}
