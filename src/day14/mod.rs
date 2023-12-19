#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 14: Parabolic Reflector Dish");
    for line in util::read_input_iter("src/day14/example_input.txt") {
        println!("{line}");
    }

}