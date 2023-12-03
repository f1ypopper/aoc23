use std::collections::HashMap;

const ROWS: usize = 140;
const COLS: usize = 140;

#[inline(always)]
fn to_number(source: &[u8]) -> u32 {
    let mut number = 0;
    for d in source {
        number = number * 10 + (d - b'0') as u32;
    }
    number
}

#[inline(always)]
fn is_symbol(c: u8) -> bool {
    !c.is_ascii_alphanumeric() && c != b'.'
}

fn check_for_symbol(source: &Vec<u8>, start: usize, end: usize) -> Option<(usize, usize)> {
    let row = start / COLS;
    let start_col = (start % COLS).checked_sub(1).unwrap_or(0);
    let end_col = ((end % COLS) + 1).min(COLS);

    let top = row.checked_sub(1).unwrap_or(0);
    let bottom = (row + 1).min(ROWS - 1);
    if row > 0 {
        for c in start_col..end_col + 1 {
            if is_symbol(source[COLS * top + c]) {
                return Some((top, c));
            }
        }
    }
    if row < ROWS - 1 {
        for c in start_col..end_col + 1 {
            if is_symbol(source[COLS * bottom + c]) {
                return Some((bottom, c));
            }
        }
    }
    if is_symbol(source[COLS * row + start_col]) {
        return Some((row, start_col));
    }
    if is_symbol(source[COLS * row + end_col]) {
        return Some((row, end_col));
    }
    None
}

fn part1(source: &Vec<u8>) -> u32 {
    let mut start;
    let mut current = 0;
    let len = source.len();
    let mut total = 0;
    while current < len {
        if source[current].is_ascii_digit() {
            start = current;
            while source[current].is_ascii_digit() {
                current += 1;
            }
            let number = to_number(&source[start..current]);
            if check_for_symbol(&source, start, current - 1).is_some() {
                total += number;
            }
        } else {
            current += 1;
        }
    }
    total
}
fn part2(source: &Vec<u8>) -> u32 {
    let mut start;
    let mut current = 0;
    let len = source.len();
    let mut total = 0;
    let mut adj = HashMap::<(usize, usize), Vec<u32>>::new();
    while current < len {
        if source[current].is_ascii_digit() {
            start = current;
            while source[current].is_ascii_digit() {
                current += 1;
            }
            let number = to_number(&source[start..current]);
            if let Some(gear) = check_for_symbol(&source, start, current - 1) {
                if source[COLS * gear.0 + gear.1] == b'*' {
                    if let Some(parts) = adj.get_mut(&gear) {
                        if parts.len() <= 1 {
                            total += parts[0] * number;
                        }
                        parts.push(number);
                    } else {
                        adj.insert(gear, vec![number]);
                    }
                }
            }
        } else {
            current += 1;
        }
    }
    total
}
fn main() {
    let mut source = std::fs::read("inputs/day3.txt").unwrap();
    source.retain(|&c| c != b'\n');
    let sol1 = part1(&source);
    let sol2 = part2(&source);
    println!("PART 1: {sol1}");
    println!("PART 2: {sol2}");
}
