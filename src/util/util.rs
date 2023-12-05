use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use ndarray::{Array2, ArrayBase, OwnedRepr, Dim};

#[allow(dead_code)]
pub fn read_input_iter(path: &str) -> impl Iterator<Item = String> {
    let input = File::open(path).expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    return lines.into_iter();
}

#[allow(dead_code)]
pub fn read_input_array(path: & str) -> ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>> {
    let input = File::open(path).expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let rows = lines.len();
    let cols: usize = lines[0].len();
    let mut array = Array2::<char>::default((rows, cols));
    for (row, line) in lines.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            array[[row, col]] = char;
        }
    }
    return array;
}