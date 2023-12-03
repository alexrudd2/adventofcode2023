use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_input(path: &str) -> impl Iterator<Item = String> {
    let input = File::open(path).expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    return lines.into_iter();
}