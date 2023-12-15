use std::{fs, cmp::{min, max}};

fn main(){
    let mut total: i64 = 0;
    
    //let file_path = "src\\advent_of_code\\2023\\data\\day_11_test.txt"; //
    let file_path = "src\\advent_of_code\\2023\\data\\day_11.txt"; // 650672493820
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map = Vec::new();
    let empty_col;
    let mut empty_row = Vec::new();
    let mut starts = Vec::new();
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        if c.contains('#') {
            find_galaxy(c, &mut starts, map.len());
        } else {
            empty_row.push(map.len());
        }
        map.push(input);
    }

    empty_col = find_empty_col(&map);

    for i in 0..starts.len() {
        for j in i..starts.len() {
            total += get_distans(starts[i], starts[j], &empty_col, &empty_row);
        }
    }

    println!("total = {}", total);
}


fn get_distans(start: (usize, usize), end: (usize, usize), empty_col: &Vec<usize>, empty_row: &Vec<usize>) -> i64 {
    let mut steps = 0;
    for ec in empty_col {
        if *ec > min(start.1, end.1) && *ec < max(start.1, end.1) {
            steps += 1000000 -1
        }
    }
    for er in empty_row {
        if *er > min(start.0, end.0) && *er < max(start.0, end.0) {
            steps += 1000000 -1
        }
    }
    steps += (start.0 as i64 - end.0 as i64).abs() + (start.1 as i64 - end.1 as i64).abs();
    steps
}


fn find_empty_col(map: &Vec<Vec<char>>) -> Vec<usize> {
    let mut empty_col = Vec::new();
    for j in 0..map[0].len() {
        let mut b = true;
        for i in 0..map.len() {
            if map[i][j] == '#' {
                b = false;
            }
        }
        if b {
            empty_col.push(j);
        }
    }
    empty_col
}


fn find_galaxy(line: &str, start: &mut Vec<(usize, usize)>, ml: usize) {
    for c in line.trim().chars().enumerate() {
        if c.1 == '#' {
            start.push((ml, c.0));
        }
    }
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
