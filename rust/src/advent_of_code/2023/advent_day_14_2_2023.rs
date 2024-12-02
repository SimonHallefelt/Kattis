use std::{fs, collections::HashSet};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_14_test.txt"; //64
    let file_path = "src\\advent_of_code\\2023\\data\\day_14.txt"; //99641
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map = Vec::new();

    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        map.push(input)
    }

    //println!("original map = \n{:?} ", map);

    map = cycle(map);
    total += calc_load(map);

    println!("total = {}", total);
}


fn cycle(old_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = old_map.clone();
    let mut hs = HashSet::new();
    let mut steps_to_loop = 0;

    println!("find start to loop");
    while hs.insert(map.clone()) {
        steps_to_loop += 1;
        map = do_cycle(map);
    }

    println!("find length of loop");
    let mut loop_leangth = 0;
    let mut hs2 = HashSet::new();
    while hs2.insert(map.clone()) {
        loop_leangth += 1;
        map = do_cycle(map);
    }

    println!("find last map");
    let cycles = 1000000000;
    let left = (cycles-steps_to_loop) - (((cycles-steps_to_loop)/loop_leangth)-1) * (loop_leangth);
    println!("steps_to_loop = {}, loop_leangth = {}, left = {}", steps_to_loop, loop_leangth, left);
    for _ in 0..left {
        map = do_cycle(map)
    }

    //return map
    map
}


fn do_cycle(mut map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    map = tilt_n(map);
    //println!("tilt_N map = \n{:?} ", map);
    map = tilt_w(map);
    //println!("tilt_W map = \n{:?} ", map);
    map = tilt_s(map);
    //println!("tilt_S map = \n{:?} ", map);
    map = tilt_e(map);
    //println!("tilt_E map = \n{:?} ", map);
    map
}


fn tilt_n(old_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = old_map.clone();
    let mut block;
    for i in 0..old_map[0].len() {
        block = -1;
        for j in 0..old_map.len() {
            match old_map[j][i] {
                '#' => block = j as i64,
                'O' => {
                    map[j][i] = '.';
                    block += 1;
                    map[block as usize][i] = 'O'; 
                },
                _ => continue,
            }
        }
    }
    map
}


fn tilt_w(old_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = old_map.clone();
    let mut block;
    for i in 0..old_map.len() {
        block = -1;
        for j in 0..old_map[0].len() {
            match old_map[i][j] {
                '#' => block = j as i64,
                'O' => {
                    map[i][j] = '.';
                    block += 1;
                    map[i][block as usize] = 'O'; 
                },
                _ => continue,
            }
        }
    }
    map
}


fn tilt_s(old_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = old_map.clone();
    let mut block: i64;
    for i in 0..old_map[0].len() {
        block = old_map.len() as i64;
        for j in (0..old_map.len()).rev() {
            match old_map[j][i] {
                '#' => block = j as i64,
                'O' => {
                    map[j][i] = '.';
                    block -= 1;
                    map[block as usize][i] = 'O'; 
                },
                _ => continue,
            }
        }
    }
    map
}


fn tilt_e(old_map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = old_map.clone();
    let mut block: i64;
    for i in 0..old_map.len() {
        block = old_map.len() as i64;
        for j in (0..old_map[0].len()).rev() {
            match old_map[i][j] {
                '#' => block = j as i64,
                'O' => {
                    map[i][j] = '.';
                    block -= 1;
                    map[i][block as usize] = 'O'; 
                },
                _ => continue,
            }
        }
    }
    map
}


fn calc_load(map: Vec<Vec<char>>) -> i64 {
    let mut load = 0;
    for i in 0..map[0].len() {
        for j in 0..map.len() {
            match map[j][i] {
                'O' => {
                    load += map.len() as i64 - j as i64; 
                },
                _ => continue,
            }
        }
    }
    load
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
