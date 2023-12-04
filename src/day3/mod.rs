#[path = "../util/util.rs"] mod util;
use ndarray::{ArrayBase, OwnedRepr, Dim, s};


pub fn main() {
    println!("Day 3: Gear Ratios");
    let schematic = util::read_input_array("src/day3/input.txt");
    let rows = schematic.shape()[0];
    let cols = schematic.shape()[1];
    let mut sum = 0;
    for row in 0..rows {
        let mut num_detected = false;
        let mut part_no: u32 = 0;
        let mut part_detected = false;
        for col in 0..cols {
            let char = schematic[[row, col]];
            if char.is_digit(10) {
                num_detected = true;
                part_no = part_no * 10 + char.to_digit(10).unwrap();
                if is_adjacent_to_symbol(&schematic, row, col, rows, cols) {
                    part_detected = true;
                }
            } else if num_detected {
                num_detected = false;
                if part_detected {
                    sum += part_no;
                    // print!("{part_no} ");
                    part_detected = false;
                }
                part_no = 0;
            }
        }
        if part_detected { // part numbers at end of row
            sum += part_no;
            // print!("{part_no} ");
        }
        // println!("\n");
    }
    println!("The sum of all part numbers is {sum}.  Somebody get the eraser!");
    let mut gear_ratio_sum = 0;
    for row in 0..rows {
        for col in 0..cols {
            let c = schematic[[row, col]];
            if c == '*' {
                // create a 3R x 7C array with gear '*' at the center [1, 3]
                let gearbox = schematic.slice(s![row - 1..row+2, col - 3..col + 4]);
                // 0 - 7 in phone order: up-left, up, up-right, left, right, down-left, down, down-right
                let mut found_numbers = vec![0; 8];
                // println!("{gearbox}");
                // up-left
                if gearbox[[0,2]].is_digit(10) && !gearbox[[0,3]].is_digit(10) {
                    if gearbox[[0,0]].is_digit(10) && gearbox[[0,1]].is_digit(10) { // 3 digits
                        found_numbers[0] = gearbox[[0,0]].to_digit(10).unwrap() * 100
                                          + gearbox[[0,1]].to_digit(10).unwrap() * 10
                                          + gearbox[[0,2]].to_digit(10).unwrap() * 1
                    } else if gearbox[[0,1]].is_digit(10) { // 2 digits {
                        found_numbers[0] = gearbox[[0,1]].to_digit(10).unwrap() * 10
                                          + gearbox[[0,2]].to_digit(10).unwrap() * 1 
                    } else { // 1 digit
                        found_numbers[0] = gearbox[[0, 2]].to_digit(10).unwrap();
                    }
                }
                // up
                if gearbox[[0,3]].is_digit(10) {
                    if gearbox[[0,1]].is_digit(10) && gearbox[[0,2]].is_digit(10) {
                        found_numbers[1] = gearbox[[0,1]].to_digit(10).unwrap() * 100
                                        + gearbox[[0,2]].to_digit(10).unwrap() * 10
                                        + gearbox[[0,3]].to_digit(10).unwrap() * 1
                    } else if gearbox[[0,2]].is_digit(10) && gearbox[[0,4]].is_digit(10) {
                        found_numbers[1] = gearbox[[0,2]].to_digit(10).unwrap() * 100
                                        + gearbox[[0,3]].to_digit(10).unwrap() * 10
                                        + gearbox[[0,4]].to_digit(10).unwrap() * 1
                    } else if gearbox[[0,4]].is_digit(10) && gearbox[[0,5]].is_digit(10) {
                        found_numbers[1] = gearbox[[0,3]].to_digit(10).unwrap() * 100
                                        + gearbox[[0,4]].to_digit(10).unwrap() * 10
                                        + gearbox[[0,5]].to_digit(10).unwrap() * 1
                    } else if gearbox[[0,2]].is_digit(10) {
                        found_numbers[1] = gearbox[[0,2]].to_digit(10).unwrap() * 10
                                        + gearbox[[0,3]].to_digit(10).unwrap() * 1
                    } else if gearbox[[0,4]].is_digit(10) {
                        found_numbers[1] = gearbox[[0,3]].to_digit(10).unwrap() * 10
                                        + gearbox[[0,4]].to_digit(10).unwrap() * 1
                    } else {
                        found_numbers[1] = gearbox[[0,3]].to_digit(10).unwrap();
                    }
                }
                // up-right
                if gearbox[[0,4]].is_digit(10) && !gearbox[[0,3]].is_digit(10) {
                    if gearbox[[0,5]].is_digit(10) && gearbox[[0,6]].is_digit(10) { // 3 digits
                        found_numbers[2] = gearbox[[0,4]].to_digit(10).unwrap() * 100
                                            + gearbox[[0,5]].to_digit(10).unwrap() * 10
                                            + gearbox[[0,6]].to_digit(10).unwrap() * 1
                    } else if gearbox[[0,5]].is_digit(10) { // 2 digits {
                        found_numbers[2] = gearbox[[0,4]].to_digit(10).unwrap() * 10
                                            + gearbox[[0,5]].to_digit(10).unwrap() * 1 
                    } else { // 1 digit
                        found_numbers[2] = gearbox[[0, 4]].to_digit(10).unwrap();
                    }
                }
                // left 
                if gearbox[[1, 2]].is_digit(10) {
                    found_numbers[3] = parse_gear_label_left(gearbox.slice(s![1,0..3;-1]).to_vec());
                }
                // right
                if gearbox[[1, 4]].is_digit(10) {
                    found_numbers[4] = parse_gear_label_right(gearbox.slice(s![1,4..7]).to_vec());
                }
                // down-left  // NEEDS TEST
                if gearbox[[2,2]].is_digit(10) && !gearbox[[2,3]].is_digit(10) {
                    found_numbers[5] = parse_gear_label_left(gearbox.slice(s![2,0..3;-1]).to_vec());
                }
                // down
                if gearbox[[2,3]].is_digit(10) {
                    if gearbox[[2,2]].is_digit(10) {
                        found_numbers[6] = parse_gear_label_right(gearbox.slice(s![2,1..5]).to_vec());
                    } else {
                        found_numbers[6] = parse_gear_label_right(gearbox.slice(s![2,2..6]).to_vec());
                    }
                }
                // down-right // NEEDS TEST
                if gearbox[[2,4]].is_digit(10) && !gearbox[[2,3]].is_digit(10) {
                    found_numbers[7] = parse_gear_label_right(gearbox.slice(s![2,4..7]).to_vec());
                }
            // println!("{found_numbers:?}");
            let gear_labels = found_numbers.iter().filter(|n| **n != 0).collect::<Vec<_>>();
            if gear_labels.len() == 2 {
                gear_ratio_sum += gear_labels[0] * gear_labels[1];
            }
            }
        }
    }
    println!("This calculation grinds my gears.  The gear ratio sum is {gear_ratio_sum}.\n");

}

fn is_adjacent_to_symbol(array: &ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>, row: usize, col: usize, rows: usize, cols: usize) -> bool {
    // up-left
    if row > 0 && col > 0 && is_symbol(array[[row - 1, col - 1]]) { return true; }
    // up
    if row > 0 && is_symbol(array[[row - 1, col]]) { return true;}
    // up-right
    if row > 0 && col < cols - 1 && is_symbol(array[[row - 1, col + 1]]) { return true; }
    // left
    if col > 0 && is_symbol(array[[row, col - 1]]) { return true; }
    // right
    if col < cols - 1 && is_symbol(array[[row, col + 1]]) { return true; }
    // down-left
    if row < rows - 1 && col > 0 && is_symbol(array[[row + 1, col - 1]]) { return true; }
    // down
    if row < rows - 1 && is_symbol(array[[row + 1, col]]) { return true; }
    // down-right
    if row < rows - 1 && col < cols - 1 && is_symbol(array[[row + 1, col + 1]]) { return true; }
    return false;
}

fn is_symbol(c:char) -> bool {
    return !(c.is_digit(10) || c == '.')
}

fn parse_gear_label_left(list: Vec<char>) -> u32 {
    let ones = list[0].to_digit(10).unwrap();
    if !list[1].is_digit(10) {
        return ones;
    }
    let tens = list[1].to_digit(10).unwrap();
    if !list[2].is_digit(10) {
        return tens * 10 + ones;
    }
    let hundreds = list[2].to_digit(10).unwrap();
    return hundreds * 100 + tens * 10 + ones;
}

fn parse_gear_label_right(list: Vec<char>) -> u32 {
    let mut sum = 0;
    for item in list {
        if item.is_digit(10) {
            sum = 10 * sum + item.to_digit(10).unwrap();
        }
    }
    return sum;
}