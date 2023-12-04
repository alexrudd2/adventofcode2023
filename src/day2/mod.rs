#[path = "../util/util.rs"] mod util;

#[derive(Debug)] struct CubeDraw(i32, i32, i32);

pub fn main() {
    println!("Day 2: Cube Conundrum");
    let mut games = Vec::new();
    for line in util::read_input("src/day2/input.txt") {
        let colon = line.find(":").unwrap();
        let gameno = &line[5..colon]
                           .parse::<i32>().unwrap();
        // println!("Game {gameno}:");
        let gameline = &line[colon+2..];
        let mut draws = Vec::<CubeDraw>::new();
        for drawlines in gameline.split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for mut draw in drawlines.split(",") {
                draw = draw.trim();
                let space = draw.find(" ").unwrap();
                let num = draw[0..space].parse::<i32>().unwrap();
                let color = draw.as_bytes()[space + 1] as char;
                match color {
                    'r' => red = num,
                    'g' => green = num,
                    'b' => blue = num,
                    _ => panic!(),
                }
            }
            let set = CubeDraw(red, green, blue);
            draws.push(set);
        }
        games.push((*gameno, draws));

    }
    // part 1
    const MAX_RED: i32 = 12;
    const MAX_GREEN: i32 = 13;
    const MAX_BLUE: i32 = 14;
    let mut total: i32 = 0;
    'outer: for game in &games {
        let gameno = game.0;
        // println!("Game {gameno}:");
        for draw in &game.1 {
            // println!("{draw:?}");
            if draw.0 > MAX_RED || draw.1 > MAX_GREEN || draw.2 > MAX_BLUE {
                // println!("impossible");
                continue 'outer
            }
        }
        total += gameno;
    }
    println!("The sum of the possible cube IDs is {total}.");

    // part 2
    total = 0;
    for game in games {
        let mut min_red: i32 = 0;
        let mut min_green: i32 = 0;
        let mut min_blue: i32 = 0;
        for draw in game.1 {
            if draw.0 > min_red { min_red = draw.0}
            if draw.1 > min_green { min_green = draw.1}
            if draw.2 > min_blue { min_blue = draw.2}
        }
        let power:i32 = min_red * min_blue * min_green;
        total += power;
    }
    println!("The total cube power is {total}.   That's over 9000!");

}