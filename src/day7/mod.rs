use std::cmp::Ordering;

#[path = "../util/util.rs"]
mod util;
#[derive(Debug)]
#[allow(dead_code)]
struct Hand {
    cards: Vec<char>,
    bid: i32,
    value: i32,
    value_joker: i32,
    rank: i32,
    rank_joker: i32,
}
pub fn main() {
    println!("Day 7: Camel Poker");
    let mut hands: Vec<Hand> = Vec::new();
    for line in util::read_input_iter("src/day7/input.txt") {
        let hand: Vec<char> = line[0..5].chars().collect();
        let bid: i32 = line[6..line.len()].parse().unwrap();
        let value = evaluate_hand(&hand);
        let value_joker = evaluate_hand_joker(&hand);
        hands.push(Hand {
            cards: hand,
            bid,
            value,
            value_joker,
            rank: 0,
            rank_joker: 0,
        });
    }
    hands.sort_by(|a, b| a.value.cmp(&b.value));

    let max_rank = hands.len();
    let mut total = 0;
    let mut i = 0;
    while i < max_rank {
        let rank = (i + 1) as i32;
        hands[i].rank = rank;
        total += hands[i].bid * rank;
        i += 1;
    }

    println!("Your total winnings are {total} ElfBux.");

    hands.sort_by(|a, b| a.value_joker.cmp(&b.value_joker));
    total = 0;
    i = 0;
    while i < max_rank {
        let rank = (i + 1) as i32;
        hands[i].rank_joker = rank;
        total += hands[i].bid * rank;
        i += 1;
    }
    // for h in &hands {
    //     println!("{h:?}");
    // }
    println!("Your total winnings with jokers are {total} ElfBux.");
    println!("Can you even spend these outside the North Pole?\n");
}

fn evaluate_hand(hand: &Vec<char>) -> i32 {
    const TYPE_VALUE: i32 = 1000000; // arbitrary, just needs to be bigger than 15^4
    let mut sorted = hand.clone();
    sorted.sort_by(|a, b| compare_card(a, b));
    // println!("{sorted:?}");

    let cards_value = card_value(&hand[0]) * 15_i32.pow(4)
        + card_value(&hand[1]) * 15_i32.pow(3)
        + card_value(&hand[2]) * 15_i32.pow(2)
        + card_value(&hand[3]) * 15
        + card_value(&hand[4]);
    // 5 of a kind
    if sorted[0] == sorted[4] {
        return 6 * TYPE_VALUE + cards_value;
    }

    // 4 of a kind
    if sorted[0] == sorted[3] || sorted[1] == sorted[4] {
        return 5 * TYPE_VALUE + cards_value;
    }

    // full house
    if sorted[0] == sorted[2] && sorted[3] == sorted[4]
        || sorted[2] == sorted[4] && sorted[0] == sorted[1]
    {
        return 4 * TYPE_VALUE + cards_value;
    }

    // 3 of a kind
    if sorted.windows(3).any(|i| i[0] == i[2]) {
        return 3 * TYPE_VALUE + cards_value;
    }

    // two pair
    if sorted[0] == sorted[1] && sorted[2] == sorted[3]
        || sorted[0] == sorted[1] && sorted[3] == sorted[4]
        || sorted[1] == sorted[2] && sorted[3] == sorted[4]
    {
        return 2 * TYPE_VALUE + cards_value;
    }

    // pair
    if sorted.windows(2).any(|i| i[0] == i[1]) {
        return 1 * TYPE_VALUE + cards_value;
    }

    // high card
    return cards_value;
}

fn evaluate_hand_joker(hand: &Vec<char>) -> i32 {
    const TYPE_VALUE: i32 = 1000000; // arbitrary, just needs to be bigger than 15^4
    let mut sorted = hand.clone();
    sorted.sort_by(|a, b| compare_card_joker(a, b));
    // println!("{sorted:?}");

    let cards_value = card_value_joker(&hand[0]) * 15_i32.pow(4)
        + card_value_joker(&hand[1]) * 15_i32.pow(3)
        + card_value_joker(&hand[2]) * 15_i32.pow(2)
        + card_value_joker(&hand[3]) * 15
        + card_value_joker(&hand[4]);
    // rely on the fact that jokers will be at the start
    // 5 of a kind
    if sorted.windows(2).all(|i| cmp_j(i[0], i[1])) {
        return 6 * TYPE_VALUE + cards_value;
    }

    // 4 of a kind (max 3 jokers or it would have been 5 of a kind)
    if sorted[2] == 'J' // 3 jokers
       || sorted[1] == 'J' && sorted[2..].windows(2).any(|i| i[0] == i[1]) // 2 jokers
       || sorted[0] == 'J' && sorted[1..].windows(3).any(|i| i[0] == i[2]) // 1 jokers
       || sorted.windows(4).any(|i| i[0] == i[3])
    // 0 jokers
    {
        return 5 * TYPE_VALUE + cards_value;
    }

    // full house (max 2 jokers)
    if cmp_j(sorted[0], sorted[2]) && cmp_j(sorted[1], sorted[2]) && sorted[3] == sorted[4] // XXXYY
      || sorted[2] == sorted[4] && cmp_j(sorted[0], sorted[1])
    // XXYYY
    {
        return 4 * TYPE_VALUE + cards_value;
    }

    // 3 of a kind (max 2 jokers)
    if sorted[1] == 'J' // 2 jokers
      || sorted[0] == 'J' && sorted[1..].windows(2).any(|i| i[0] == i[1]) // 1 joker
      || sorted.windows(3).any(|i | i[0] == i[2])
    // 0 jokers
    {
        return 3 * TYPE_VALUE + cards_value;
    }

    // two pair (max 1 joker)
    if sorted[0] == 'J' && sorted[1..].windows(2).any(|i| i[0] == i[1] ) // 1 joker
      || sorted[0] == sorted[1] && sorted[2] == sorted[3] // 0 jokers
      || sorted[0] == sorted[1] && sorted[3] == sorted[4]
      || sorted[1] == sorted[2] && sorted[3] == sorted[4]
    {
        return 2 * TYPE_VALUE + cards_value;
    }

    // pair
    if sorted[0] == 'J' // 1 joker
      || sorted.windows(2).any(|i| i[0] == i[1])
    // 0 jokers
    {
        return 1 * TYPE_VALUE + cards_value;
    }

    // high card
    return cards_value;
}

fn compare_card(a: &char, b: &char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    return card_value(a).cmp(&card_value(b));
}

fn compare_card_joker(a: &char, b: &char) -> Ordering {
    if a == b {
        return Ordering::Equal;
    }
    return card_value_joker(a).cmp(&card_value_joker(b));
}

fn card_value(card: &char) -> i32 {
    return match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    };
}

fn card_value_joker(card: &char) -> i32 {
    return match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as i32,
    };
}

fn cmp_j(a: char, b: char) -> bool {
    return a == b || a == 'J' || b == 'J';
}
