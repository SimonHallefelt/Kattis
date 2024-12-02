use std::{fs, collections::{HashMap, VecDeque, HashSet}};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_21_test.txt"; // 16 , 6 steps
    let file_path = "src\\advent_of_code\\2023\\data\\day_21.txt"; // 3646 , 64 steps
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = Vec::new();
    let mut start = (0, 0);
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        map.push(input.clone());
        if c.contains("S") {
            start = (map.len() as i32 -1, find_start(input));
            println!("hej, start = {:?}", start);
        }
    }

    total += run(&map, start);

    println!("total = {}", total);
}


fn run(map: &Vec<Vec<char>>, start: (i32,i32)) -> i64 {
    let mut reach = HashSet::new();
    let dir = Vec::from([(0,1),(0,-1),(1,0),(-1,0)]);
    reach.insert(start);
    let steps = 64;

    for _ in 0..steps {
        let mut reach_new = HashSet::new();
        for r in reach {
            for d in &dir {
                let pos = (r.0+d.0, r.1+d.1);
                if pos.0 < 0 || pos.1 < 0 || 
                    pos.0 >= map.len() as i32 || pos.1 >= map[0].len() as i32 {
                    continue;
                }
                if map[pos.0 as usize][pos.1 as usize] != '#' {
                    reach_new.insert((pos.0, pos.1));
                }
            }
        }
        reach = reach_new;
        //println!("reach = {:?}", reach);
    }

    //println!("reach = {:?}", reach);

    reach.len() as i64
}


fn find_start(input: Vec<char>) -> i32 {
    for i in 0..input.len() {
        if input[i] == 'S' {
            return i as i32;
        }
    }
    i32::MAX
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
