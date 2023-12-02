fn check_game(gid: u32, game: &Vec<Vec<(usize, &str)>>) -> Option<u32> {
    for set in game {
        for (count, color) in set {
            if *color == "blue" {
                if *count > 14 {
                    return None;
                }
            } else if *color == "red" {
                if *count > 12 {
                    return None;
                }
            } else {
                if *count > 13 {
                    return None;
                }
            }
        }
    }
    Some(gid)
}
fn part1(source: &String) -> u32 {
    source
        .lines()
        .map(|line| {
            let mut game = line.split(": ");
            let gid = game
                .next()
                .unwrap()
                .split(' ')
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let sets = game
                .next()
                .unwrap()
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(|ball| -> (usize, &str) {
                            let bd = ball.split(' ').collect::<Vec<&str>>();
                            (bd[0].parse::<usize>().unwrap(), bd[1])
                        })
                        .collect::<Vec<(usize, &str)>>()
                })
                .collect::<Vec<Vec<(usize, &str)>>>();
            (gid, sets)
        })
        .filter_map(|(gid, sets)| check_game(gid, &sets))
        .sum()
}

fn min_power_set(game: &Vec<Vec<(usize, &str)>>) -> u32 {
    let mut red = 1;
    let mut green = 1;
    let mut blue = 1;
    game.iter().for_each(|set| {
        set.iter().for_each(|(count, color)| {
            if *color == "red" {
                if *count > red {
                    red = *count;
                }
            } else if *color == "green" {
                if *count > green {
                    green = *count;
                }
            } else {
                if *count > blue {
                    blue = *count;
                }
            }
        });
    });

    (red * green * blue).try_into().unwrap()
}

fn part2(source: &String) -> u32 {
    source
        .lines()
        .map(|line| {
            let mut game = line.split(": ");
            let gid = game
                .next()
                .unwrap()
                .split(' ')
                .skip(1)
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let sets = game
                .next()
                .unwrap()
                .split("; ")
                .map(|set| {
                    set.split(", ")
                        .map(|ball| -> (usize, &str) {
                            let bd = ball.split(' ').collect::<Vec<&str>>();
                            (bd[0].parse::<usize>().unwrap(), bd[1])
                        })
                        .collect::<Vec<(usize, &str)>>()
                })
                .collect::<Vec<Vec<(usize, &str)>>>();
            (gid, sets)
        })
        .map(|(_, sets)| min_power_set(&sets))
        .sum()
}

fn main() {
    let source = std::fs::read_to_string("inputs/day2.txt").unwrap();
    let sol1 = part1(&source);
    let sol2 = part2(&source);
    println!("PART 1: {sol1}");
    println!("PART 2: {sol2}");
}
