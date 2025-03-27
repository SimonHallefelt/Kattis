use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_6.txt"; //4663
    let map = read_input(file_path);

    let mut start = (0,0);
    'outer: for i in 0..map.len() {
        for j in 0..map.get(i).unwrap().len() {
            if '^' == *map.get(i).unwrap().get(j).unwrap() {
                start = (i as i64, j as i64);
                break 'outer;
            }
        }
    }

    let visited = run(&map, start);

    println!("{}", visited);
}

fn run(map: &Vec<Vec<char>>, mut pos: (i64, i64)) -> usize {
    let mut dir = 0;
    let dirs = vec![(-1,0), (0, 1), (1, 0), (0, -1)];
    let mut visited = HashSet::new();
    loop {
        visited.insert(pos);
        let d = dirs.get(dir).unwrap();
        let new_pos = (pos.0 + d.0, pos.1 + d.1);
        if new_pos.0 < 0 || new_pos.0 >= map.len() as i64 {
            break;
        }
        if new_pos.1 < 0 || new_pos.1 >= map.get(new_pos.0 as usize).unwrap().len() as i64 {
            break;
        }
        println!("{:?}, {:?}, {:?}, {:?}", new_pos, dir, map.len(), map.get(new_pos.0 as usize).unwrap().len());
        if '#' == *map.get(new_pos.0 as usize).unwrap().get(new_pos.1 as usize).unwrap() {
            dir = (dir + 1) % 4;
        } else {
            pos = new_pos;
        }
    }
    visited.len()
}

fn read_input(file: &str) -> Vec<Vec<char>> {
    let content = read_to_string(file).expect("could not read file");
    content.trim().split("\n").map(|a| a.trim().chars().collect()).collect()
}
