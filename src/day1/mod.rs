use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn main() {
    let input = File::open("src/day1/input.txt").expect("Could not read input");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().collect();

    let mut total1: u32 = 0;
    let mut total2: u32 = 0;
    for line in lines {
        let calibration1: String = line.expect("Could not parse line").trim().to_string();
        // strings can overlap
        let calibration2 = calibration1.replace("zero", "z0o")
                                         .replace("one", "o1e")
                                         .replace("two", "t2o")
                                         .replace("three", "t3e")
                                         .replace("four", "f4r")
                                         .replace("five", "f5e")
                                         .replace("six", "s6x")
                                         .replace("seven", "s7n")
                                         .replace("eight", "e8t")
                                         .replace("nine", "n9e");
        let mut first_num: u32 = 11;
        let mut last_num: u32 = 11;
        for char in calibration1.chars() {

            if !char.is_numeric() {
                continue;
            }
            if first_num == 11 {
                first_num = char.to_digit(10).expect("char is not a digit")
            }
            last_num = char.to_digit(10).expect("char is not a digit")
        }
        total1 += first_num * 10 + last_num;
        // println!("{} {}", first_num, last_num);

        let mut first_num: u32 = 11;
        let mut last_num: u32 = 11;
        for char in calibration2.chars() {

            if !char.is_numeric() {
                continue;
            }
            if first_num == 11 {
                first_num = char.to_digit(10).expect("char is not a digit")
            }
            last_num = char.to_digit(10).expect("char is not a digit")
        }
        total2 += first_num * 10 + last_num;
    }
    println!("Day 1: Trebuchet?!");
    println!("The sum of the calibration values (no words) is {}.", total1);
    println!("The sum of the calibration values (with words) is {}.", total2);

}