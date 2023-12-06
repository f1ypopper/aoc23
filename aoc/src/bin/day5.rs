fn part1(source: &String) -> usize {
    let mut lines = source.split("\n\n");
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap());
    let maps = lines
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|l| {
                    let map = l
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>();
                    (map[1]..map[1] + map[2], map[0])
                })
                .collect::<Vec<(std::ops::Range<usize>, usize)>>()
        })
        .collect::<Vec<Vec<(std::ops::Range<usize>, usize)>>>();
    seeds
        .map(|seed| {
            maps.iter().fold(seed, |value, map| {
                map.iter()
                    .find_map(|(range, destination)| {
                        range.contains(&value).then(|| {
                            ((*destination as i64 - range.start as i64) + value as i64) as usize
                        })
                    })
                    .unwrap_or(value)
            })
        })
        .min()
        .unwrap()
}

fn part2(source: &String) -> usize {
    let mut lines = source.split("\n\n");
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let maps = lines
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|l| {
                    let map = l
                        .split_whitespace()
                        .map(|n| n.parse().unwrap())
                        .collect::<Vec<usize>>();
                    (map[1]..map[1] + map[2], map[0])
                })
                .collect::<Vec<(std::ops::Range<usize>, usize)>>()
        })
        .collect::<Vec<Vec<(std::ops::Range<usize>, usize)>>>();
    seeds
        .chunks_exact(2)
        .flat_map(|s| {
            let seedstart = s[0];
            let length = s[1];
            seedstart..seedstart + length
        })
        .map(|seed| {
            maps.iter().fold(seed, |value, map| {
                map.iter()
                    .find_map(|(range, destination)| {
                        range.contains(&value).then(|| {
                            ((*destination as i64 - range.start as i64) + value as i64) as usize
                        })
                    })
                    .unwrap_or(value)
            })
        })
        .min()
        .unwrap()
}
fn main() {
    let source = std::fs::read_to_string("inputs/day5.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART1: {sol1}");
    let sol2 = part2(&source);
    println!("PART2: {sol2}");
}
