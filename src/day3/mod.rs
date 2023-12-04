#[path = "../util/util.rs"] mod util;
use ndarray::{Array2, ArrayBase, OwnedRepr, Dim};


pub fn main() {
    println!("Day 3: Gear Ratios");
    let schematic = util::read_input_array("src/day3/input.txt");
    let rows = schematic.shape()[0];
    let cols = schematic.shape()[1];
    let mut sum = 0;
    for row in 0..rows {
        let mut num_detected = false;
        let mut part_no: i32 = 0;
        let mut part_detected = false;
        for col in 0..cols {
            let char = schematic[[row, col]];
            if char.is_digit(10) {
                num_detected = true;
                part_no = part_no * 10 + char.to_digit(10).unwrap() as i32;
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
        if part_detected { // part numberss at end of row
            sum += part_no;
            // print!("{part_no} ");
        }
        // println!("\n");
    }
    println!("The sum of all part numbers is {sum}.  Somebody get the eraser!");
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