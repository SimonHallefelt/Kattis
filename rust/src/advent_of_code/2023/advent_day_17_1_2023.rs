use std::{fs, cmp::Ordering, collections::{BinaryHeap, HashSet, HashMap}};

#[derive(Clone, Eq, PartialEq)]
struct Node {
    cost: i64,
    pos: (i64,i64),
    dir: (i64,i64),
    same_dir: i64,
    path: Vec<(i64,i64)>,
}


impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}


impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_17_test.txt"; //102
    let file_path = "src\\advent_of_code\\2023\\data\\day_17.txt"; //1155
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = Vec::new();
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        map.push(input);
    }

    //println!("map =\n {:?}", map);
    total += run(map);

    println!("total = {}", total);
}


fn run(map: Vec<Vec<i64>>) -> i64 {
    let mut t = 0;
    let mut final_path = Vec::new();
    let directions = Vec::from([(0,1), (0,-1), (1,0), (-1,0)]);
    let mut visited = HashSet::new();
    let mut walking = BinaryHeap::new();
    walking.push(Node {cost: 0, pos: (0,0), dir: (0,0), same_dir: 0, path: Vec::new()});

    while let Some(Node {cost, pos, dir, same_dir, path}) = walking.pop() {
        if same_dir == 3 {continue;}
        if !visited.insert((pos, dir, same_dir)) {continue;}
        println!("hej, cost = {}, pos = {:?}", cost, pos);
        if pos.0+1 == map.len() as i64 && pos.1+1 == map[0].len() as i64 {t = cost; final_path = path.clone(); break;}

        for d in &directions {
            let new_pos = (pos.0+d.0, pos.1+d.1);
            if new_pos.0 < 0 || new_pos.1 < 0 {continue;}
            if new_pos.0 >= map.len() as i64 || new_pos.1 >= map[0].len() as i64 {continue;}
            if dir.0 * -1 == d.0 && dir.1 * -1 == d.1 as i64 {continue;}
            

            let new_cost = cost+map[new_pos.0 as usize][new_pos.1 as usize];
            let mut new_path = path.clone();
            new_path.push(new_pos);
            if dir == *d {
                if visited.contains(&(new_pos, dir, same_dir+1)) {continue;}
                walking.push(Node {cost: new_cost, pos: new_pos, dir: dir, same_dir: same_dir+1, path: new_path});
            } else {
                if visited.contains(&(new_pos, *d, 0)) {continue;}
                walking.push(Node {cost: new_cost, pos: new_pos, dir: *d, same_dir: 0, path: new_path});
            }
        }
    }

    println!("final_path = \n {:?}", final_path);
    t
}


fn fix_input(line: &str) -> Vec<i64> {
    line.trim().chars().map(|x| x.to_digit(10).unwrap() as i64).collect()
}
