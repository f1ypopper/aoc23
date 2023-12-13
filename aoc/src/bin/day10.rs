const COLS: usize = 140;

fn calc(pos: (i32, i32))->usize{
    pos.0 as usize *COLS+pos.1 as usize
}

#[derive(Debug)]
enum Dir{
    Right,
    Up,
    Down,
    Left
}

struct Grid{
    grid: Vec<char>
}

impl Grid{
    fn new(source: &String) -> Self{
        Grid{grid: source.chars().filter(|&c| c != '\n').collect::<Vec<char>>()}
    }

    fn start(&self) -> (i32, i32){
        self.grid
        .iter()
        .enumerate()
        .find_map(|(pos, &c)| (c == 'S').then_some(((pos / COLS) as i32, (pos % COLS) as i32)))
        .unwrap()
    }

    fn get(&self, pos: (i32, i32))->char{
        if pos.0 < 0 || pos.1 < 0{
            '.'
        }else{
            self.grid[calc(pos)]
        }
    }
}
fn part1(source: &String) -> usize {
    let grid = Grid::new(source);
    let start = grid.start();
    //println!("START: {start:?}");
    let (mut pos, mut dir) = if matches!(grid.get((start.0, start.1+1)), '-'){
        ((start.0, start.1+1), Dir::Right)
    }else if matches!(grid.get((start.0, start.1-1)), '-' | 'L'){
        ((start.0, start.1-1), Dir::Left)
    }else if matches!(grid.get((start.0+1, start.1)), '|' | 'L' | 'J'){
        ((start.0+1, start.1), Dir::Down)
    }else if matches!(grid.get((start.0-1, start.1)), '|' | '7' | 'F'){
        ((start.0-1, start.1), Dir::Up)
    }else{
        unimplemented!()
    };

    let mut steps = 1;
    loop{
        //println!("POS: {pos:?} DIR: {dir:?} {}", grid.get(pos));
        match (grid.get(pos), dir){
            ('|', Dir::Down)=>{pos = (pos.0+1, pos.1); dir = Dir::Down;}
            ('|', Dir::Up)=>{pos = (pos.0-1, pos.1); dir = Dir::Up;}
            ('J', Dir::Down)=>{pos = (pos.0, pos.1-1); dir = Dir::Left;},
            ('J', Dir::Right)=>{pos = (pos.0-1, pos.1); dir = Dir::Up;},
            ('L', Dir::Down)=>{pos = (pos.0, pos.1+1); dir = Dir::Right;},
            ('L', Dir::Left)=>{pos = (pos.0-1, pos.1); dir = Dir::Up;},
            ('-', Dir::Left)=>{pos = (pos.0, pos.1-1); dir = Dir::Left;},
            ('-', Dir::Right)=>{pos = (pos.0, pos.1+1); dir = Dir::Right;},
            ('F', Dir::Left)=>{pos = (pos.0+1, pos.1); dir = Dir::Down;},
            ('F', Dir::Up)=>{pos = (pos.0,pos.1+1); dir = Dir::Right;},
            ('7', Dir::Right)=>{pos = (pos.0+1,pos.1); dir = Dir::Down;},
            ('7', Dir::Up)=>{pos = (pos.0,pos.1-1); dir = Dir::Left;},
            ('S', _)=>break,
            _=>unimplemented!(),
        }
        steps+=1;
    }

    steps / 2
}

fn main() {
    let source = std::fs::read_to_string("inputs/day10.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART 1: {sol1}");
}
