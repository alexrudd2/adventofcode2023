use std::collections::HashMap;

#[path = "../util/util.rs"]
mod util;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

pub fn main() {
    println!("Day 19: Aplenty");
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    let mut accepted_parts: HashMap<Part, i32> = HashMap::new();
    let mut reached_middle = false;
    for line in util::read_input_iter("src/day19/input.txt") {
        let len = line.len();
        if len == 0 { reached_middle = true; continue; }
        if !reached_middle {
            let i = line.find('{').unwrap();
            let name = &line[0..i].to_string();
            let workflow: Vec<_> = line[i+1..len-1].split(',').map(|s|s.to_string()).collect();
            workflows.insert(name.clone(), workflow.clone());
        } else {
            let values: Vec<_> = line[1..len-1].split(',').map(|s| s[2..].parse::<i32>().unwrap()).collect();
            let part = Part {
                x: values[0],
                m: values[1],
                a: values[2],
                s: values[3]
            };
            parts.push(part);
        }
    }

    for part in parts{
        println!("{part:?}");
        let mut next_workflow = String::from("in");
        'outer: loop {
            let workflow = workflows.get(&next_workflow).unwrap();
            'inner: for task in workflow {
                // print!("{task} ");
                if task.as_str() == "R" {
                    break 'outer;
                } else if task.as_str() == "A" {
                    let value = part.x + part.m + part.a + part.s;
                    accepted_parts.insert(part.clone(), value);
                    break 'outer;
                } else {
                    if !task.contains(':') {
                        next_workflow = task.to_string();
                    } else {
                        let colon = task.find(':').unwrap();
                        let destination = &task[colon+1..].to_string();
                        let conditional = &task[..colon];
                        if conditional.contains('<') {
                            let operator = conditional.find('<').unwrap();
                            let test = &conditional[0..operator];
                            let value: i32 = conditional[operator+1..].parse().unwrap();
                            // print!("|CONDL If {test} < {value} go {destination} |");
                            if test == "x" && part.x < value
                               || test == "m" && part.m < value
                               || test == "a" && part.a < value
                               || test == "s" && part.s < value 
                            { 
                                if destination == "R" { break 'outer }
                                if destination == "A" {
                                    let value = part.x + part.m + part.a + part.s;
                                    accepted_parts.insert(part.clone(), value);
                                    break 'outer
                                }
                                next_workflow = destination.clone();
                                break 'inner 
                            }
                        } else {
                            let operator = conditional.find('>').unwrap();
                            let test = &conditional[0..operator];
                            let value: i32 = conditional[operator+1..].parse().unwrap();
                            // print!("|CONDL If {test} > {value} go {destination} |");
                            if test == "x" && part.x > value
                               || test == "m" && part.m > value
                               || test == "a" && part.a > value
                               || test == "s" && part.s > value 
                            { 
                                if destination == "R" { break 'outer }
                                if destination == "A" {
                                    let value = part.x + part.m + part.a + part.s;
                                    accepted_parts.insert(part.clone(), value);
                                    break 'outer
                                }
                                next_workflow = destination.clone();
                                break 'inner 
                            }
                        }
                    }
                }
            }
            print!("{next_workflow} ");
            // break;
        }
        print!("\n");
    }
    let mut total = 0;
    for a in &accepted_parts {
        total += a.1;
    }
    println!("{total}");

}