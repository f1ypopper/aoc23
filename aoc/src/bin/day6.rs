fn part1(source: &String) -> usize {
    let mut lines = source.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|t| t.parse::<usize>().unwrap());
    let distance = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|d| d.parse::<usize>().unwrap());
    times
        .zip(distance)
        .map(|(time, distance)| {
            (1..time)
                .filter(|t| {
                    let speed = t;
                    let time = time - t;
                    speed * time > distance
                })
                .count()
        })
        .fold(1, |ways, w| ways * w)
}

fn part2(source: &String) -> usize {
    let mut lines = source.lines();
    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .map(|s| s.replace(" ", "").parse::<usize>().unwrap())
        .unwrap();
    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .map(|s| s.replace(" ", "").parse::<usize>().unwrap())
        .unwrap();
    (1..time)
        .filter(|t| {
            let speed = t;
            let time = time - t;
            speed * time > distance
        })
        .count()
}

fn main() {
    let source = std::fs::read_to_string("inputs/day6.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART 1: {sol1}");
    let sol2 = part2(&source);
    println!("PART 2: {sol2}");
}
