#[path = "../util/util.rs"]
mod util;
use std::collections::HashMap;
use num::integer::lcm;

pub fn main() {
    println!("Day 8: Haunted Wasteland");
    let mut iterator = util::read_input_iter("src/day8/input.txt");
    let directions = iterator.next().unwrap();
    iterator.next();

    let mut network = HashMap::new();
    for line in iterator {
        let node = line[0..3].to_string();
        let left = line[7..10].to_string();
        let right = line[12..15].to_string();
        network.insert(node, (left, right));
        // if node.chars().last().unwrap() == 'A' {   // damn it borrow checker
        //     positions.push(&node[..]);
        // }
    }
    // println!("{network:?}");
    let mut position: &str = "AAA";
    let mut steps = 0;
    let len = directions.len();
    loop {
        let char = directions.chars().nth(steps % len).unwrap();
        steps += 1;
        match char {
            'L' => position = &network[position].0,
            'R' => position = &network[position].1,
            _ => panic!()
        }
        if position == "ZZZ" { break };
    }
    println!("You can escape the maze in {steps} steps.  Make sure to feed the camel!");

    let mut positions = vec!["VCA", "FRA", "SNA", "HNA", "AAA", "RPA"];

    let len = directions.len();
    let mut total_steps = 1;
    for i in 0..6 {
        let mut steps = 0;
        loop {
            let char = directions.chars().nth(steps % len).unwrap();
            steps += 1;
            match char {
                'L' => positions[i] = &network[positions[i]].0,
                'R' => positions[i] = &network[positions[i]].1,
                _ => panic!()
            }
            if positions[i].chars().nth(2).unwrap() == 'Z' { break };
        }
        total_steps = lcm(total_steps, steps as i64);
    }
    println!("The ghosts need more steps (floats?), a total of {total_steps}.\n");
}
