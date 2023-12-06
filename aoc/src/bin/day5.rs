#[derive(Debug)]
struct Range {
    source: usize,
    destination: usize,
    length: usize,
}
impl Range {
    fn new(m: &str) -> Range {
        let splitm = m
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        Range {
            source: splitm[1],
            destination: splitm[0],
            length: splitm[2],
        }
    }
}
fn part1(source: &String) -> usize {
    let mut lines = source.split("\n\n");
    let seeds = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut maps = Vec::<Vec<Range>>::new();
    lines.for_each(|line| {
        maps.push(
            line.split('\n')
                .skip(1)
                .map(|m| Range::new(m))
                .collect::<Vec<Range>>(),
        );
    });
    seeds
        .iter()
        .map(|seed| {
            let mut value = *seed;
            maps.iter().for_each(|map| {
                value = map
                    .iter()
                    .find_map(|r| {
                        (value >= r.source && value <= r.source + r.length).then(|| {
                            ((r.destination as i64 - r.source as i64) + value as i64) as usize
                        })
                    })
                    .unwrap_or(value);
            });
            value
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
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let mut maps = Vec::<Vec<Range>>::new();
    lines.for_each(|line| {
        maps.push(
            line.split('\n')
                .skip(1)
                .map(|m| Range::new(m))
                .collect::<Vec<Range>>(),
        );
    });
    seeds
        .chunks_exact(2)
        .map(|rn| {
            (rn[0]..rn[0] + rn[1]).into_iter().map(|seed| {
                let mut value = seed;
                maps.iter().for_each(|map| {
                    value = map
                        .iter()
                        .find_map(|r| {
                            (value >= r.source && value <= r.source + r.length).then(|| {
                                ((r.destination as i64 - r.source as i64) + value as i64) as usize
                            })
                        })
                        .unwrap_or(value);
                });
                value
            }).min().unwrap()
        })
        .min()
        .unwrap()
}
fn main() {
    let source = std::fs::read_to_string("inputs/day5.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART1: {sol1}");
    let sol2 = part2(&source);
    println!("PART1: {sol2}");
}
