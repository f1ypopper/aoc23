use std::fs;

const NUMS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn part1(source: &String) -> u32 {
    let lines = source.lines();
    lines
        .map(|line| -> u32 {
            line.chars()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap()
                * 10
                + line
                    .chars()
                    .rev()
                    .find(|c| c.is_digit(10))
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
        })
        .sum()
}

fn part2(source: &String) -> u32 {
    let lines = source.lines();
    lines
        .map(|line| {
            let (mut apos, mut a) = line
                .char_indices()
                .find(|(_, c)| c.is_digit(10))
                .map(|(pos, c)| (pos, c.to_digit(10).unwrap()))
                .unwrap();
            let (mut bpos, mut b) = line
                .char_indices().rev()
                .find(|(_, c)| c.is_digit(10))
                .map(|(pos, c)| (pos, c.to_digit(10).unwrap()))
                .unwrap();

            let first = apos;
            let last = bpos;

            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = line[..first].find(num) {
                    if pos < apos {
                        apos = pos;
                        a = (idx + 1) as u32;
                    }
                }
            }

            for (idx, num) in NUMS.iter().enumerate() {
                if let Some(pos) = line[last..].rfind(num) {
                    if pos + last > bpos {
                        bpos = pos + last;
                        b = (idx + 1) as u32;
                    }
                }
            }

            a * 10 + b
        })
        .sum()
}
fn main() {
    let source = fs::read_to_string("inputs/day1.txt").unwrap();
    let sol1 = part1(&source);
    let sol2 = part2(&source);
    println!("PART 1: {sol1}");
    println!("PART 2: {sol2}");
}
