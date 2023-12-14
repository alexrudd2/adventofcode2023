#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 12: Hot Springs");
    let mut springs = Vec::new();
    let mut info = Vec::new();
    for line in util::read_input_iter("src/day12/example_input.txt") {
        let v: Vec<&str> = line.split(" ").collect();
        let s = v[0].to_string();
        let i = v[1].to_string();
        let j: Vec<i32> = i.split(",").flat_map(|s| s.parse::<i32>()).collect();
        springs.push(s);
        info.push(j);
    }
    // let mut combos = -0;

    println!("{springs:?}, {info:?}");
}