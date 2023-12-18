use std::{fs, cmp::Ordering, collections::{BinaryHeap, HashSet, HashMap}};


fn main() {
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_18_test.txt"; //62
    let file_path = "src\\advent_of_code\\2023\\data\\day_18.txt"; //40745
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut instructions = Vec::new();
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        instructions.push(input);
    }

    //println!("map =\n {:?}", map);
    total += run(instructions);

    println!("total = {}", total);
}


fn run(instructions: Vec<Vec<String>>) -> i64 {
    let mut map = Vec::new();
    let size = 6000;
    for i in 0..size {
        map.push(Vec::new());
        for _ in 0..size {
            map[i].push(true);
        }
    }
    println!("built map");

    let mut steps = 0;
    let mut pos = (size as i32/2, size as i32/2);
    map[pos.0 as usize][pos.1 as usize] = false;
    let directions = HashMap::from([("R".to_string(), (0,1)), ("L".to_string(), (0,-1)), ("U".to_string(), (1,0)), ("D".to_string(), (-1,0))]);
    for i in instructions {
        let dir = directions.get(&i[0]).unwrap();
        let dis = i[1].parse::<usize>().unwrap();

        steps += dis;
        for _ in 0..dis {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            map[pos.0 as usize][pos.1 as usize] = false;
        }
    }
    println!("walked map");

    bfs(&mut map);
    println!("bfs map");

    let mut total = 0;
    for i in 0..size {
        for j in 0..size {
            if map[i][j] {
                total += 1
            }
        }
    }
    total += steps as i32;
    println!("calc map");

    total as i64
}


fn bfs(map: &mut Vec<Vec<bool>>) {
    let mut to_visit = Vec::new();
    to_visit.push((0,0));
    let mut start;
    while !to_visit.is_empty() {
        start = to_visit.pop().unwrap();
        if start.0 > 0 && map[start.0-1][start.1] {
            map[start.0-1][start.1] = false;
            to_visit.push((start.0-1, start.1));
        }
        if start.0+1 < map.len() && map[start.0+1][start.1] {
            map[start.0+1][start.1] = false;
            to_visit.push((start.0+1, start.1));
        }
        if start.1 > 0 && map[start.0][start.1-1] {
            map[start.0][start.1-1] = false;
            to_visit.push((start.0, start.1-1));
        }
        if start.1+1 < map[0].len() && map[start.0][start.1+1] {
            map[start.0][start.1+1] = false;
            to_visit.push((start.0, start.1+1));
        } 
    }
}


fn fix_input(line: &str) -> Vec<String> {
    line.trim().split(" ").map(|x| x.to_string()).collect()
}
