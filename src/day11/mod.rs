#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 11: Cosmic Expansion");
    let mut input_image = Vec::new();
    for line in util::read_input_iter("src/day11/input.txt") {
        let v: Vec<char> = line.split_whitespace().flat_map(|i| i.chars()).collect();
        input_image.push(v);
    }
    let expanded_rows_image = expand_empty_rows(&input_image);
    let expanded_cols_image = expand_empty_rows(&transpose(&expanded_rows_image));
    let image = transpose(&expanded_cols_image);

    let rows = image.len();
    let cols = image[0].len();
    let mut galaxies = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let c = image[row][col];
            if c == '#' {
                galaxies.push((row as i32, col as i32));
            }
        }
    }

    let mut galaxy_pairs = Vec::new();
    let mut total = 0;
    for (i, &x) in galaxies.iter().enumerate() {
        for &y in galaxies.iter().skip(i + 1) {
            galaxy_pairs.push((x, y));
            total += num::abs(x.0 - y.0);
            total += num::abs(x.1 - y.1);
        }
    }

    println!("The distance between all the galaxies is {total}.");

}

fn transpose<T: Clone + std::default::Default>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if matrix.is_empty() || matrix[0].is_empty() {
        return Vec::new();
    }

    let num_rows = matrix.len();
    let num_cols = matrix[0].len();

    let mut transposed = vec![vec![Default::default(); num_rows]; num_cols];

    for i in 0..num_rows {
        for j in 0..num_cols {
            transposed[j][i] = matrix[i][j].clone();
        }
    }

    transposed
}
fn expand_empty_rows(input: &Vec<Vec<char>>) -> Vec<Vec<char>>{
    let mut output: Vec<Vec<char>> = Vec::new();
    for row in input.iter() {
        if !row.contains(&'#') {
            output.push(row.clone());
        }
        output.push(row.to_vec());
    }
    output
}