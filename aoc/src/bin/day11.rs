fn both(source: &String, times: usize) -> usize{
    let width = source.chars().position(|c| c == '\n').unwrap();
    let height = source.lines().count();
    let space = source.chars().filter(|&c| c != '\n').collect::<Vec<char>>();
    let galaxies: Vec<(usize, usize)> = space
        .iter()
        .enumerate()
        .filter(|(_, &c)| c == '#')
        .map(|(pos, _)| (pos / width, pos % width))
        .collect();

    let empty_rows: Vec<usize> = source
        .lines()
        .enumerate()
        .filter_map(|(pos, line)| line.chars().all(|c| c == '.').then_some(pos))
        .collect();

    let empty_cols: Vec<usize> = (0..width)
        .filter_map(|col| {
            (col..height * width + col)
                .step_by(width)
                .all(|i| space[i] == '.')
                .then_some(col)
        })
        .collect();

    (0..galaxies.len()-1).map(|i|{
        let a = galaxies[i];
        (i+1..galaxies.len()).map(|j|{
            let b = galaxies[j];
            let gaps = empty_rows.iter().filter(|ex|{
                (a.0.min(b.0)..a.0.max(b.0)).contains(ex)
            }).count()+ empty_cols.iter().filter(|ey|{
                (a.1.min(b.1)..a.1.max(b.1)).contains(ey)
            }).count();
            a.0.abs_diff(b.0)+a.1.abs_diff(b.1)+gaps*times
        }).sum::<usize>()
    }).sum()
}

fn main() {
    let source = std::fs::read_to_string("inputs/day11.txt").unwrap();
    println!("PART 1: {}", both(&source, 1));
    println!("PART 2: {}", both(&source, 999999));
}
