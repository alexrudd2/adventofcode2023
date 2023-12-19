#[path = "../util/util.rs"]
mod util;
use indexmap::IndexMap;

pub fn main() {
    println!("Day 15: Lens Library");
    let line = util::read_input_iter("src/day15/input.txt").next().unwrap();
    let steps = line.split(',');
    let mut total = 0;
    let mut boxes: Vec<IndexMap<&str, i32>> = Vec::with_capacity(256);
    for _ in 0..256 {
        boxes.push(IndexMap::new());
    }

    for step in steps {

        total += calc_hash(step);

        let len = step.len();
        if step.contains('=') {
            let lens = &step[0..len - 2];
            let focal = step[len - 1..].parse::<i32>().unwrap();
            boxes[calc_hash(lens)].insert(lens, focal);
        } else if step.contains('-') {
            let lens = &step[0..len - 1];
            boxes[calc_hash(lens)].shift_remove(lens);
        }
    }
    println!("The total is {total}");

    let part2_total = &boxes.iter().enumerate().fold(0, |acc, (index, value)| {
        acc + (index + 1) as i32
            * value
                .values()
                .enumerate()
                .fold(0, |acc, (index, &value)| acc + (index + 1) as i32 * value)
    });
    println!("The total focal length of all those lenses in boxes is {part2_total:?}");
}

fn calc_hash(s: &str) -> usize {
    let mut hash = 0;
    for c in s.as_bytes() {
        let c = *c as i32;
        hash += c;
        hash *= 17;
        hash = hash % 256;
    }
    hash as usize
}
