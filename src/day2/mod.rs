#[path = "../util/util.rs"] mod util;

#[derive(Debug)] struct CubeDraw(i8, i8, i8);

pub fn main() {
    println!("Day 2: Cube Conundrum");
    const MAX_RED: i8 = 12;
    const MAX_GREEN: i8 = 13;
    const MAX_BLUE: i8 = 14;
    let mut total: i32 = 0;
    let mut games = Vec::new();
    'outer: for line in util::read_input("src/day2/input.txt") {
        let colon = line.find(":").unwrap();
        let gameno = &line[5..colon]
                           .parse::<i32>().unwrap();
        println!("Game {gameno}:");
        games.push((*gameno, Vec::<CubeDraw>::new()));
        let gameline = &line[colon+2..];
        for draws in gameline.split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for mut draw in draws.split(",") {
                draw = draw.trim();
                let space = draw.find(" ").unwrap();
                let num = draw[0..space].parse::<i8>().unwrap();
                let color = draw.as_bytes()[space + 1] as char;
                match color {
                    'r' => red = num,
                    'g' => green = num,
                    'b' => blue = num,
                    _ => panic!(),
                }
            }
            let set = CubeDraw(red, green, blue);
            if red > MAX_RED || blue > MAX_BLUE || green > MAX_GREEN {
                println!("Impossible\n");
                continue 'outer;
            }
            print!("{set:?}");
            // println!("r:{red}, g:{green}, b:{blue}");
            print!("\n");
        }
        total += gameno;

    }
    println!("The sum of the total cube IDs is {total}.");
}