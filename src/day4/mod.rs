#[path = "../util/util.rs"]
mod util;

#[derive(Debug, Clone)]
struct Scratchcard(i32, i32, Vec<i32>, Vec<i32>);

pub fn main() {
    println!("Day 4: Scratchcards");
    let mut cards = Vec::new();
    for line in util::read_input_iter("src/day4/input.txt") {
        let colon = line.find(":").unwrap();
        let cardno = &line[5..colon].trim().parse::<i32>().unwrap();
        let pipe = line.find("|").unwrap();
        let winline = &line[colon + 2..pipe - 1];
        let haveline = &line[pipe + 2..];
        let winvec: Vec<i32> = str_to_i32_vec(winline);
        let havevec: Vec<i32> = str_to_i32_vec(haveline);
        cards.push(Scratchcard(*cardno, 1, winvec, havevec));
    }
    let mut total = 0;
    for card in &cards {
        let mut points = 0;
        for have_num in &card.3 {
            if !card.2.contains(&have_num) {
                continue;
            }
            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
            // println!("WINNER: {have_num} makes card worth {points}");
        }
        total += points;
    }
    println!("The elf's scratchcards are worth {total}.");

    let mut i = 0;
    let mut total_cards = 0;
    while i < cards.len() {
        let mut copies: usize = 0;
        let card = &cards[i];
        let current_copies = cards[i].1;
        total_cards += current_copies;
        for have_num in &card.3 {
            if card.2.contains(&have_num) {
                copies += 1;
            }
        }
        // println!("C-: {cards:?}\n");
        for j in 1..copies + 1 {
            cards[i + j].1 += current_copies;
        }
        // println!("C+: {cards:?}\n");
        i += 1;
    }
    println!("I got coal instead of cards this year. =( The number of cards on my wishlist was {total_cards}.\n");
}

fn str_to_i32_vec(s: &str) -> Vec<i32> {
    // todo: use generics?
    let v: Vec<&str> = s.split_whitespace().collect();
    return v
        .iter()
        .map(|&s| s.parse().expect("Tried to parse non-integer value"))
        .collect();
}
