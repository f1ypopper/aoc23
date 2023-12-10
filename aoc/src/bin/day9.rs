fn compute_next_from_first(values: &Vec<i64>) -> i64 {
    if values.iter().all(|&v| v == 0) {
        return 0;
    }
    let delta = values
        .windows(2)
        .map(|ab| ab[1] - ab[0])
        .collect::<Vec<i64>>();
    -compute_next_from_first(&delta) + values.first().unwrap()
}
fn compute_next_from_last(values: &Vec<i64>) -> i64 {
    if values.iter().all(|&v| v == 0) {
        return 0;
    }
    let delta = values
        .windows(2)
        .map(|ab| ab[1] - ab[0])
        .collect::<Vec<i64>>();
    compute_next_from_last(&delta) + values.last().unwrap()
}
fn part1(source: &String) -> i64 {
    source
        .lines()
        .map(|line| {
            compute_next_from_last(
                &line
                    .split_whitespace()
                    .map(|d| d.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum()
}
fn part2(source: &String) -> i64 {
    source
        .lines()
        .map(|line| {
            compute_next_from_first(
                &line
                    .split_whitespace()
                    .map(|d| d.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            )
        })
        .sum()
}
fn main() {
    let source = std::fs::read_to_string("inputs/day9.txt").unwrap();
    println!("PART 1: {}", part1(&source));
    println!("PART 2: {}", part2(&source));
}
