#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 13: Point of Incidence");
    let mut patterns = Vec::new();
    let mut transposed_patterns = Vec::new();
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
    let mut h_reflections = vec![usize::MAX; patterns.len()];
    let mut v_reflections = vec![usize::MAX; patterns.len()];

    // let mut h_reflections_new = vec![usize::MAX; patterns.len()];
    // let mut v_reflections_new = vec![usize::MAX; patterns.len()];

    for (j, pattern) in patterns.iter().enumerate() {
        
        let reflection_indices = check_horizonal_reflect(pattern);
        if reflection_indices.len() > 0 {
            total += (reflection_indices[0] + 1) * 100;
            h_reflections[j] = reflection_indices[0];
        }

        let chars_vec: Vec<_> = pattern
            .iter()
            .map(|i| i.clone().chars().collect())
            .collect();
        let transposed_pattern = transpose(&chars_vec);
        transposed_patterns.push(transposed_pattern.clone());

        // println!("{transposed_pattern:?}");
 
        let reflection_indices = check_vertical_reflect(&transposed_pattern);
        if reflection_indices.len() > 0 {
            total += (reflection_indices[0] + 1) * 1;
            v_reflections[j] = reflection_indices[0];
        }
        // println!("Vertical: {reflects_vertically}");
        // println!("{j}\n");
    }

    println!("The notes summarize to a value of {total}.");
    println!("you gonna remember all that?");

    let mut part2_total = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        let rows = pattern.len();
        let cols = pattern[0].len();
        let mut new_pattern = pattern.clone();
        'outer: for row in 0..rows {
            for col in 0..cols {
                let line = new_pattern[row].clone();
                let mut str = String::new();
                let char = line.chars().nth(col).unwrap();
                str.push_str(&line[0..col]);
                if char == '.' {
                    str.push_str("#");
                } else {
                    str.push_str(".");
                }
                str.push_str(&line[col + 1..]);
                // println!("{line} -> {str}");
                new_pattern[row] = str;
                let reflect_indices = check_horizonal_reflect(&new_pattern);
                if reflect_indices.len() > 0 {
                    for reflect_index in reflect_indices {
                        if v_reflections[i] != usize::MAX || reflect_index != h_reflections[i] {
                            // print!("{reflect_index} is a ");
                            // let old_h = h_reflections[i];
                            // let old_v = v_reflections[i];
                            // print!("NEW horiz reflect for pattern {i} ");
                            // println!("old h: {old_h} old v: {old_v}");
                            part2_total += (reflect_index + 1) * 100;
                            // h_reflections_new[i] = reflect_index;
                            break 'outer;
                        }
                    }
                }
                new_pattern[row] = line; 
            }
        }
    }
    for (i, pattern) in transposed_patterns.iter().enumerate() {
        let rows = pattern.len();
        let cols = pattern[0].len();
        let mut new_pattern = pattern.clone();
        'outer: for row in 0..rows {
            for col in 0..cols {
                let char = pattern[row][col];
                if char == '.' {
                    new_pattern[row][col] = '#';
                } else {
                    new_pattern[row][col] = '.';
                }
                let reflection_indices = check_vertical_reflect(&new_pattern);
                if reflection_indices.len() > 0 {
                    for reflect_index in reflection_indices {
                        if (h_reflections[i] != usize::MAX) || (reflect_index != v_reflections[i]) {
                            // print!("{reflect_index} is a ");
                            // let old_h = h_reflections[i];
                            // let old_v = v_reflections[i];
                            // print!("NEW vertical reflect for pattern {i} ");
                            // println!("old h: {old_h} old v: {old_v}");
                            part2_total += (reflect_index + 1) * 1;
                            // v_reflections_new[i] = reflect_index;
                            break 'outer;
                        }
                    }
                }
                new_pattern[row][col] = char;
            }
        }
    }
    // for (i, _) in patterns.iter().enumerate() {
    //     print!("\n{i}: old: ");
    //     if v_reflections[i] != usize::MAX { print!("V") }
    //     if h_reflections[i] != usize::MAX { print!("H") }
    //     print!(", new: ");
    //     if v_reflections_new[i] != usize::MAX { print!("V") }
    //     if h_reflections_new[i] != usize::MAX { print!("H") }
    // }
    println!("The smudgified total is {part2_total}.");
    // 48400 is too high
    // 22029 is too low
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

fn check_horizonal_reflect(pattern: &Vec<String>) -> Vec<usize> {

    let len = pattern.len();
    let mut reflect_indices = Vec::new();
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
            reflect_indices.push(*match_index);
        }
    }
    reflect_indices
}

fn check_vertical_reflect (transposed_pattern: &Vec<Vec<char>>) -> Vec<usize> {

    let len = transposed_pattern.len();
    let mut reflect_indices = Vec::new();
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
                reflect_indices.push(*match_index);
            }
        }
    }
    reflect_indices
}