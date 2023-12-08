use std::{cmp::Ordering, collections::HashMap};
fn strength(card: &str) -> usize {
    let mut freq = HashMap::<char, usize>::new();
    card.chars().for_each(|c| {
        *freq.entry(c).or_insert(0) += 1;
    });
    let same = *freq.values().max().unwrap();
    //if same == 5{
    //    println!("FIVE OF A KIND");
    //}else if same == 4{
    //    println!("FOUR OF A KIND");
    //}else if same == 3 && freq.len() == 2{
    //    println!("FULL HOUSE");
    //}else if same == 3 && freq.len() == 3{
    //    println!("THREE OF A KIND");
    //}else if same == 2 && freq.len() == 4{
    //    println!("ONE PAIR");
    //}else if same == 1 && freq.len() == 5{
    //    println!("HIGH CARD");
    //}else{
    //    println!("TWO PAIR");
    //}
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

fn card_value(c: char) -> usize {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
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
    pub fn new(cards: &'a str, bid: usize) -> Hand {
        Hand {
            cards,
            bid,
            strength: strength(cards),
        }
    }
    pub fn compare(&self, other: &Self) -> Ordering {
        if self.strength == other.strength {
            let mut i = 0;
            loop {
                let h1 = card_value(self.cards.chars().nth(i).unwrap());
                let h2 = card_value(other.cards.chars().nth(i).unwrap());
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
            Hand::new(card, bid.parse::<usize>().unwrap())
        })
        .collect::<Vec<Hand>>();
    hands.sort_by(|a, b| a.compare(b));
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
}
