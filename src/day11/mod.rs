#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 11: Cosmic Expansion");
    let mut input_image = Vec::new();
    for line in util::read_input_iter("src/day11/input.txt") {
        let v: Vec<char> = line.split_whitespace().flat_map(|i| i.chars()).collect();
        input_image.push(v);
    }
    // let expanded_rows_image = expand_empty_rows(&input_image);
    // let expanded_cols_image = expand_empty_rows(&transpose(&expanded_rows_image));
    // let image = transpose(&expanded_cols_image);
    let image = input_image;

    let mut expanded_rows = Vec::new();
    for (i, row) in image.iter().enumerate() {
        if !row.contains(&'#') {
            expanded_rows.push(i);
        }
    }
    let mut expanded_cols = Vec::new();
    for (i, row) in transpose(&image).iter().enumerate() {
        if !row.contains(&'#') {
            expanded_cols.push(i);
        }
    }

    let rows = image.len();
    let cols = image[0].len();
    let mut galaxies = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            let c = image[row][col];
            if c == '#' {
                galaxies.push((row as i64, col as i64));
            }
        }
    }

    let mut galaxy_pairs = Vec::new();
    let mut total1 = 0;
    let mut total2 = 0;
    for (i, &x) in galaxies.iter().enumerate() {
        for &y in galaxies.iter().skip(i + 1) {
            galaxy_pairs.push((x, y));
            total1 += num::abs(x.0 - y.0);
            total2 += num::abs(x.0 - y.0);
            for i in &expanded_rows {
                if (*i as i64) > std::cmp::min(x.0, y.0) && (*i as i64) < std::cmp::max(x.0, y.0) {
                    // println!("{x:?} {i} {y:?}");
                    total1 += 2 - 1;
                    total2 += 1000000 - 1;
                }
            }
            for i in &expanded_cols {
                if (*i as i64) > std::cmp::min(x.1, y.1) && (*i as i64) < std::cmp::max(x.1, y.1) {
                    // println!("{x:?} {i} {y:?}");
                    total1 += 2 - 1;
                    total2 += 1000000 - 1;
                }
            }
            total1 += num::abs(x.1 - y.1);
            total2 += num::abs(x.1 - y.1);
        }
    }

    println!("The distance between all the galaxies is {total1}.");
    println!("The distance between all the galaxies is {total2}.");
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
// was used for Part 1, replaced
// fn expand_empty_rows(input: &Vec<Vec<char>>) -> Vec<Vec<char>>{
//     let mut output: Vec<Vec<char>> = Vec::new();
//     for row in input.iter() {
//         if !row.contains(&'#') {
//             output.push(row.clone());
//         }
//         output.push(row.to_vec());
//     }
//     output
// }
