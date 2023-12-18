#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 13: Point of Incidence");
    let mut patterns = Vec::new();
    let mut c = Vec::new();
    for line in util::read_input_iter("src/day13/input.txt") {
        if line == "" {
            patterns.push(c);
            c = vec![];
        } else {
            c.push(line);
        }
    }
    patterns.push(c);

    let mut total = 0;

    for (j, pattern) in patterns.iter().enumerate() {
        let len = pattern.len();

        let mut reflects_horizontally = false;
        let matches: Vec<_> = pattern
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] == w[1])
            .map(|(index, _)| index)
            .collect();
        if matches.len() > 0 {
            'outer: for match_index in &matches {
                let mut i = 0;
                // println!("{match_index}");
                while 1 + match_index - i > 0 as usize && match_index + i < len - 1 {
                    let a = &pattern[match_index - i];
                    let b = &pattern[match_index + 1 + i];
                    if a != b {
                        continue 'outer;
                    }
                    // println!("{i}: {a} + {b} ");

                    i += 1;
                }
                reflects_horizontally = true;
                total += (match_index + 1) * 100;
            }
        }
        // println!("Horizontal: {reflects_horizontally}");

        let mut reflects_vertically = false;
        let chars_vec: Vec<_> = pattern
            .iter()
            .map(|i| i.clone().chars().collect())
            .collect();
        let transposed_pattern = transpose(&chars_vec);
        let len = transposed_pattern.len();

        // println!("{transposed_pattern:?}");

        let matches: Vec<_> = transposed_pattern
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] == w[1])
            .map(|(index, _)| index)
            .collect();
        if matches.len() > 0 {
            'outer: for match_index in &matches {
                let mut i = 0;
                let mut break_occured = false;
                while 1 + match_index - i > 0 as usize && match_index + i < len - 1 {
                    let a = &transposed_pattern[match_index - i];
                    let b = &transposed_pattern[match_index + 1 + i];
                    if a != b {
                        // println!("> {i}: {a:?} + {b:?} ");
                        break_occured = true;
                        continue 'outer;
                    }
                    // println!("{i}: {a:?} + {b:?} ");

                    i += 1;
                }
                if !break_occured {
                    reflects_vertically = true;
                    total += (match_index + 1) * 1;
                }
            }
        }
        // println!("Vertical: {reflects_vertically}");
        // println!("{j}\n");
        if !reflects_horizontally && !reflects_vertically {
            panic!();
        }
    }

    println!("The notes summarize to a value of {total}.");
    println!("you gonna remember all that?\n");
}

// fn reverse_str(input: &str) -> String {
//     let reversed: String = input.chars().rev().collect();
//     reversed
// }

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
