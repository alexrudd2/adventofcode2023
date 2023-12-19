#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 18: Lavaduct Lagoon");
    for line in util::read_input_iter("src/day18/example_input.txt") {
        println!("{line}");
    }

}