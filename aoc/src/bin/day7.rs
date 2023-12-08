use std::{cmp::Ordering, collections::HashMap};
fn strength(card: &str) -> usize {
    let mut freq = HashMap::<char, usize>::new();
    card.chars().for_each(|c| {
        *freq.entry(c).or_insert(0) += 1;
    });
    let same = *freq.values().max().unwrap();
    match (same, freq.len()) {
        (5, _) => 7, //FIVE OF A KIND
        (4, _) => 6, //FOUR OF A KIND
        (3, 2) => 5, //FULL HOUSE
        (3, 3) => 4, //THREE OF A KIND
        (2, 4) => 2, //ONE PAIR
        (1, 5) => 0, //HIGH CARD
        _ => 3,      //TW0 PAIR
    }
}
fn joker_strength(card: &str) -> usize {
    let mut freq = HashMap::<char, usize>::new();
    card.chars().filter(|c| c != &'J').for_each(|c| {
        *freq.entry(c).or_insert(0) += 1;
    });
    let highest = freq
        .iter()
        .max_by(|a, b| a.1.cmp(b.1))
        .map(|(k, _)| k)
        .unwrap_or(&'J');
    let wild = card.chars().filter(|c| c == &'J').count();
    *(freq.entry(*highest).or_insert(0)) += wild;
    let same = *freq.values().max().unwrap();
    match (same, freq.len()) {
        (5, _) => 7, //FIVE OF A KIND
        (4, _) => 6, //FOUR OF A KIND
        (3, 2) => 5, //FULL HOUSE
        (3, 3) => 4, //THREE OF A KIND
        (2, 4) => 2, //ONE PAIR
        (1, 5) => 0, //HIGH CARD
        _ => 3,      //TW0 PAIR
    }
}
fn card_value(c: char, joker: bool) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => if joker {0}else{11},
        'T' => 10,
        _ => c.to_digit(10).unwrap() as usize,
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str,
    bid: usize,
    strength: usize,
}

impl<'a> Hand<'a> {
    pub fn new(cards: &'a str, bid: usize, joker:bool) -> Hand {
        Hand {
            cards,
            bid,
            strength: if joker {joker_strength(cards)}else{strength(cards)},
        }
    }
    pub fn compare(&self, other: &Self, joker: bool) -> Ordering {
        if self.strength == other.strength {
            let mut i = 0;
            loop {
                let h1 = card_value(self.cards.chars().nth(i).unwrap(), joker);
                let h2 = card_value(other.cards.chars().nth(i).unwrap(), joker);
                if h1 != h2 {
                    return h1.cmp(&h2);
                }
                i += 1;
            }
        } else {
            self.strength.cmp(&other.strength)
        }
    }
}

fn part1(source: &String) -> usize {
    let mut hands = source
        .lines()
        .map(|line| {
            let mut l = line.split_whitespace();
            let card = l.next().unwrap();
            let bid = l.next().unwrap();
            Hand::new(card, bid.parse::<usize>().unwrap(), false)
        })
        .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.compare(b, false));
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}
fn part2(source: &String) -> usize {
    let mut hands = source
        .lines()
        .map(|line| {
            let mut l = line.split_whitespace();
            let card = l.next().unwrap();
            let bid = l.next().unwrap();
            Hand::new(card, bid.parse::<usize>().unwrap(), true)
        })
        .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.compare(b, true));
    hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bid)
        .sum()
}
fn main() {
    let source = std::fs::read_to_string("inputs/day7.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART 1: {sol1}");
    let sol2 = part2(&source);
    println!("PART 2: {sol2}");
}
