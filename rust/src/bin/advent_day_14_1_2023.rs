use std::{fs, cmp::{min, max}};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "C:\\Users\\simon\\Documents\\Simon\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_14_test.txt"; //136
    let file_path = "C:\\Users\\simon\\Documents\\Simon\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_14.txt"; //106997
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map = Vec::new();

    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        map.push(input)
    }

    total += calc_load(map);

    println!("total = {}", total);
}


fn calc_load(map: Vec<Vec<char>>) -> i64 {
    let mut load = 0;
    let mut block;
    for i in 0..map[0].len() {
        block = -1;
        for j in 0..map.len() {
            match map[j][i] {
                '#' => block = j as i64,
                'O' => {
                    load += map.len() as i64 - (block+1); 
                    block += 1;
                    //println!("      hej, i = {}, load = {}", i, load);
                },
                _ => continue,
            }
        }
        println!("hej, i = {}, load = {}", i, load);
    }
    load
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
