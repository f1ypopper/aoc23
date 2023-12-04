use std::collections::HashSet;
fn part1(source: &String) -> usize {
    source
        .lines()
        .map(|line| line.split(':').nth(1).unwrap().split_once('|').unwrap())
        .map(|(winning, given)| {
            let mut win_set = HashSet::new();
            winning.trim().split_whitespace().for_each(|number| {
                win_set.insert(number.parse::<usize>().unwrap());
            });
            given
                .trim()
                .split_whitespace()
                .filter(|number| win_set.contains(&number.parse::<usize>().unwrap()))
                .count()
        }).filter(|&matches| matches > 0).fold(0, |acc, matches|{
            acc+2_usize.pow(matches as u32 -1)
        })
}
fn part2(source: &String) -> usize {
    let mut cards = vec![1; 211];
    source
        .lines()
        .map(|line| line.split(':').nth(1).unwrap().split_once('|').unwrap())
        .enumerate().for_each(|(cn, (winning, given))| {
            let mut win_set = HashSet::new();
            winning.trim().split_whitespace().for_each(|number| {
                win_set.insert(number.parse::<usize>().unwrap());
            });
            let matches = given
                .trim()
                .split_whitespace()
                .filter(|number| win_set.contains(&number.parse::<usize>().unwrap()))
                .count();
            for i in cn+1..cn+matches+1{
                cards[i]+=cards[cn];
            }

        });
    cards.iter().sum()
}
fn main() {
    let source = std::fs::read_to_string("inputs/day4.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART 1: {sol1}");
    let sol2 = part2(&source);
    println!("PART 2: {sol2}");
}
