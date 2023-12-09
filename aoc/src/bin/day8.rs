fn part1(source: &String)->usize{
    let mut split_source = source.split("\n\n");
    let directions = split_source.next().unwrap().as_bytes();  
    let len = directions.len();  
    let mut map = std::collections::HashMap::new();
    split_source.next().unwrap().lines().for_each(|line|{
        let node = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(node, (left, right));
    });
    let mut current = "AAA";
    let mut steps = 0;
    let mut i = 0;
    while current != "ZZZ"{
        let (l,r) = map.get(current).unwrap();
        if directions[i%len] == b'L'{
            current = l;
        }else{
            current = r;
        }
        i+=1;
        steps+=1;
    }
    steps
}

fn main(){
    let source = std::fs::read_to_string("inputs/day8.txt").unwrap();
    let sol1 = part1(&source);
    println!("PART 1: {sol1}");
}