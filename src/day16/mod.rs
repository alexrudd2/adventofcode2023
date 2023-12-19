#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 16: The Floor Will Be Lava");
    for line in util::read_input_iter("src/day16/example_input.txt") {
        println!("{line}");
    }
}
