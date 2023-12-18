use geo::Contains;
use geo::{Coord, LineString, Polygon};
use ndarray::Array2;
#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 10: Pipe Maze");
    let tiles = util::read_input_array("src/day10/input.txt");
    // println!("{tiles:?}");
    let animal_location = find_animal(&tiles, 'S');
    let check_l = check_adjacent_pipe(&tiles, 'l', animal_location);
    let check_u = check_adjacent_pipe(&tiles, 'u', animal_location);
    let check_r = check_adjacent_pipe(&tiles, 'r', animal_location);
    let check_d = check_adjacent_pipe(&tiles, 'd', animal_location);
    // println!("{animal_location:?} -> Left: {check_l:?} Up: {check_u:?} Right: {check_r:?} Down: {check_d:?}");

    let mut pipevec = vec![Coord {
        y: animal_location.0 as f64,
        x: animal_location.1 as f64,
    }];
    let mut steps = 0;
    let mut location: (usize, usize);
    let mut last_move;
    if check_u {
        location = (animal_location.0 - 1, animal_location.1);
        last_move = 'u';
    } else if check_l {
        location = (animal_location.0 + 0, animal_location.1 - 1);
        last_move = 'l';
    } else if check_d {
        location = (animal_location.0 + 1, animal_location.1);
        last_move = 'd';
    } else if check_r {
        location = (animal_location.0 + 0, animal_location.1 + 1);
        last_move = 'r';
    } else {
        panic!("No adjacent pipe found");
    }
    steps += 1;
    pipevec.push(Coord {
        y: location.0 as f64,
        x: location.1 as f64,
    });
    while location != animal_location {
        (location, last_move) = move_location(&tiles, location, last_move);
        steps += 1;
        pipevec.push(Coord {
            y: location.0 as f64,
            x: location.1 as f64,
        });
    }
    steps /= 2;
    println!("It will take {steps} steps to get to the farthest point.");

    let pipeline = LineString(pipevec);
    let pipepolygon = Polygon::new(pipeline, vec![]);
    let mut total_enclosed = 0;
    for row in 0..tiles.shape()[0] {
        for col in 0..tiles.shape()[1] {
            let coord = Coord {
                x: col as f64,
                y: row as f64,
            };
            let contains = pipepolygon.contains(&coord);
            if contains {
                // print!("I");
                total_enclosed += 1;
            } else {
                //print!{"O"};
            }
        }
        // print!("\n");
    }
    println!("There are {total_enclosed} points enclosed by the pipeline.\n");
}

fn find_animal(array: &Array2<char>, target: char) -> (usize, usize) {
    for (row, row_iter) in array.outer_iter().enumerate() {
        for (col, &ch) in row_iter.iter().enumerate() {
            // Check if the current element is equal to the target character
            if ch == target {
                return (row, col);
            }
        }
    }
    panic!("Animal not found");
}

fn check_adjacent_pipe(tiles: &Array2<char>, direction: char, location: (usize, usize)) -> bool {
    match direction {
        'u' => {
            if location.0 == 0 {
                return false;
            }
            let target = tiles[[location.0 - 1, location.1]];
            return target == '7' || target == '|' || target == 'F' || target == 'S';
        }
        'l' => {
            if location.1 == 0 {
                return false;
            }
            let target = tiles[[location.0, location.1 - 1]];
            return target == 'L' || target == '-' || target == 'F' || target == 'S';
        }
        'd' => {
            if location.0 == tiles.shape()[0] - 1 {
                return false;
            }
            let target = tiles[[location.0 + 1, location.1]];
            return target == 'J' || target == '|' || target == 'L' || target == 'S';
        }
        'r' => {
            if location.1 == tiles.shape()[1] - 1 {
                return false;
            }
            let target = tiles[[location.0, location.1 + 1]];
            return target == 'J' || target == '-' || target == '7' || target == 'S';
        }
        _ => {
            panic!("Invalid direction")
        }
    }
}

fn move_location(
    tiles: &Array2<char>,
    location: (usize, usize),
    last_move: char,
) -> ((usize, usize), char) {
    let current = tiles[location];
    let mut check_l = false;
    let mut check_u = false;
    let mut check_r = false;
    let mut check_d = false;
    if last_move != 'r' && (current == 'J' || current == '-' || current == '7') {
        check_l = check_adjacent_pipe(&tiles, 'l', location)
    }
    if last_move != 'd' && (current == 'J' || current == '|' || current == 'L') {
        check_u = check_adjacent_pipe(&tiles, 'u', location)
    }
    if last_move != 'l' && (current == 'L' || current == '-' || current == 'F') {
        check_r = check_adjacent_pipe(&tiles, 'r', location)
    }
    if last_move != 'u' && (current == '7' || current == '|' || current == 'F') {
        check_d = check_adjacent_pipe(&tiles, 'd', location)
    }

    // println!("{location:?} -> Left: {check_l:?} Up: {check_u:?} Right: {check_r:?} Down: {check_d:?}");

    if check_u {
        return ((location.0 - 1, location.1), 'u');
    } else if check_l {
        return ((location.0 + 0, location.1 - 1), 'l');
    } else if check_d {
        return ((location.0 + 1, location.1), 'd');
    } else if check_r {
        return ((location.0 + 0, location.1 + 1), 'r');
    } else {
        panic!("No adjacent pipe found");
    }
}
