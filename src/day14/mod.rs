use std::collections::HashMap;

use ndarray::{ArrayBase, Dim, OwnedRepr};

#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 14: Parabolic Reflector Dish");
    let mut rocks = util::read_input_array("src/day14/input.txt");
    slide_up(&mut rocks);
    let total_load_up = calc_total_load(&mut rocks);
    println!("The total load after sliding north once is {total_load_up}.");

    let mut cycle_loads = HashMap::new();
    let mut cycle = 0;
    let big_modulo = 1000000000 % 7;
    let mut cycle_repeated = false;
    #[allow(unused_assignments)]
    let mut total_load = 0;
    loop {
        cycle += 1;
        slide_up(&mut rocks);
        slide_left(&mut rocks);
        slide_down(&mut rocks);
        slide_right(&mut rocks);
        total_load = calc_total_load(&mut rocks);
        // println!("cycle {cycle} has load {total_load}");
        if cycle_loads.contains_key(&rocks) {
            if cycle_repeated && cycle % 7 == big_modulo {
                break;
            } else {
                cycle_repeated = true;
            }
        } else {
            cycle_loads.insert(rocks.clone(), cycle % 7);
        }
    }
    println!("The total load after 1000000000 cycles is {total_load}.");
    println!("Do rocks get dizzy?!");
}

fn calc_total_load(rocks: &mut ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>) -> usize {
    let mut total_load = 0;
    let rows = rocks.shape()[0];
    let cols = rocks.shape()[1];
    for col in 0..cols {
        for row in 0..rows {
            let current = rocks[[row, col]];
            if current == 'O' {
                let load = rows - row;
                // println!("({row}, {col}) has load {load}");
                total_load += load;
            }
        }
    }
    total_load
}

fn slide_up(rocks: &mut ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>) {
    let rows = rocks.shape()[0];
    let cols = rocks.shape()[1];
    for col in 0..cols {
        for row in 1..rows {
            let current = rocks[[row, col]];
            let above = rocks[[row - 1, col]];
            if current == 'O' {
                if above == '.' {
                    let mut r = row - 1;
                    'move_rock: while r > 0 {
                        let above = rocks[[r - 1, col]];
                        if above != '.' {
                            break 'move_rock;
                        }
                        r -= 1;
                    }
                    rocks[[row, col]] = '.';
                    rocks[[r, col]] = 'O';
                    // println!(" ({row}, {col}) -> ({r}, {col})\n");
                }
            }
        }
    }
}

fn slide_down(rocks: &mut ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>) {
    let rows = rocks.shape()[0];
    let cols = rocks.shape()[1];
    for col in 0..cols {
        for row in (0..rows - 1).rev() {
            let current = rocks[[row, col]];
            let below = rocks[[row + 1, col]];
            if current == 'O' {
                if below == '.' {
                    let mut r = row + 1;
                    'move_rock: while r < rows - 1 {
                        let below = rocks[[r + 1, col]];
                        if below != '.' {
                            break 'move_rock;
                        }
                        r += 1;
                    }
                    rocks[[row, col]] = '.';
                    rocks[[r, col]] = 'O';
                    // println!(" ({row}, {col}) -> ({r}, {col})\n");
                }
            }
        }
    }
}

fn slide_left(rocks: &mut ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>) {
    let rows = rocks.shape()[0];
    let cols = rocks.shape()[1];
    for row in 0..rows {
        for col in 1..cols {
            let current = rocks[[row, col]];
            let left = rocks[[row, col - 1]];
            if current == 'O' {
                if left == '.' {
                    let mut c = col - 1;
                    'move_rock: while c > 0 {
                        let left = rocks[[row, c - 1]];
                        if left != '.' {
                            break 'move_rock;
                        }
                        c -= 1;
                    }
                    rocks[[row, col]] = '.';
                    rocks[[row, c]] = 'O';
                    // println!(" ({row}, {col}) -> ({row}, {c})\n");
                }
            }
        }
    }
}

fn slide_right(rocks: &mut ArrayBase<OwnedRepr<char>, Dim<[usize; 2]>>) {
    let rows = rocks.shape()[0];
    let cols = rocks.shape()[1];
    for row in 0..rows {
        for col in (0..cols - 1).rev() {
            let current = rocks[[row, col]];
            let right = rocks[[row, col + 1]];
            if current == 'O' {
                if right == '.' {
                    let mut c = col + 1;
                    'move_rock: while c < cols - 1 {
                        let right = rocks[[row, c + 1]];
                        if right != '.' {
                            break 'move_rock;
                        }
                        c += 1;
                    }
                    rocks[[row, col]] = '.';
                    rocks[[row, c]] = 'O';
                    // println!(" ({row}, {col}) -> ({row}, {c})\n");
                }
            }
        }
    }
}
