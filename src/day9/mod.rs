#[path = "../util/util.rs"]
mod util;

pub fn main() {
    println!("Day 9: Mirage Maintenance");
    let mut report = Vec::new();
    for line in util::read_input_iter("src/day9/input.txt") {
        let history = vec![str_to_i32_vec(&line)];
        report.push(history);
    }
    // println!("{report:?}");

    let mut after_total = 0;
    let mut before_total = 0;
    for mut line in report {
        let mut i = 0;
        loop {
            let mut next_line = Vec::new();
            for j in line[i].windows(2) {
                next_line.push(j[1] - j[0]);
            }
            line.push(next_line.clone());
            i += 1;
            if next_line.iter().all(|k|*k==0) {
                break;
            }
        }
        // println!("{line:?}");
        for i in &line {
            after_total += i.iter().last().unwrap();
        }
        let mut before = 0;
        for i in line.iter().rev() {
            before = i[0] - before;

        }
        before_total += before;
    }
    println!("The total extrapolated at the end is {after_total}");
    println!("The total extrapolated before is {before_total}\n");
}

fn str_to_i32_vec(s: &str) -> Vec<i64> {
    // todo: use generics?
    let v: Vec<&str> = s.split_whitespace().collect();
    return v
        .iter()
        .map(|&s| s.parse().expect("Tried to parse non-integer value"))
        .collect();
}