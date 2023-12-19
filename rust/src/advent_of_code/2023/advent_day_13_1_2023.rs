use std::{fs, usize};

fn main(){
    let mut total: i64 = 0;
    
    //let file_path = "src\\advent_of_code\\2023\\data\\day_13_test.txt"; //405
    let file_path = "src\\advent_of_code\\2023\\data\\day_13.txt"; //27202
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map = Vec::new();
    let mut t = 0;
    for c in contents.trim().split("\n") {
        if c.trim().len() == 0 {
            println!("---");
            t = find_reflection(map);
            total += t;
            map = Vec::new();
            if t == 0 {
                println!("!!!!!!!!!!!!_____FEL_____!!!!!!!!!!!!!!!!!");
                return;
            }
            println!("total = {}", total);
            println!("---");
            continue;
        }
        let input = fix_input(c);
        println!("{:?}", input);
        map.push(input);
    }
    println!("---");
    t = find_reflection(map);
    total += t;
    if t == 0 {
        println!("!!!!!!!!!!!!_____FEL_____!!!!!!!!!!!!!!!!!");
        return;
    }

    println!("total = {}", total);
}

fn find_reflection(map: Vec<Vec<char>>) -> i64{
    let mut reflections = 0;



    // rows
    println!("try row");
    for i in 0..map.len() as usize {
        reflections += row_reflection(&map, i);
        if reflections != 0 {
            return reflections * 100;
        }
    }

    // cols
    println!("try col");
    for i in 0..map[0].len() as usize {
        reflections += col_reflection(&map, i);
        if reflections != 0 {
            return reflections;
        }
    }

    reflections
}


fn col_reflection(map: &Vec<Vec<char>>, index: usize) -> i64 {
    //println!("hej33, index = {}", index);
    for i in 0..map.len() {
        if !same_col(&map[i], index) {
            return 0;
        }
    }
    println!("same_col, index = {}", index);
    if index*2 <= map[0].len()-1 {
        return (index+1) as i64;
    } else {
        return index as i64;
    }
    //return (index+1) as i64;
}


fn same_col(row: &Vec<char>, index: usize) -> bool {
    if index*2 < row.len()-1 {
        //println!("hej3, index = {}", index);
        for i in 0..index+1 {
            if row[i] != row[index+index+1-i] {
                return false;
            }
        }
    } else if index*2 == row.len()-1 {
        for i in 1..index+1 {
            if row[i] != row[index+index+1-i] {
                return false;
            }
        }
    } else {
        println!("hej4, index = {}", index);
        for i in index..row.len() {
            if row[i] != row[index-1-(i-index)] {
                return false;
            }
        }
    }
    true
}


fn row_reflection(map: &Vec<Vec<char>>, index: usize) -> i64 {
    if index*2 < map.len()-1 {
        //println!("hej1, index = {}", index);
        for i in 0..index+1 {
            if !same_row(&map[i], &map[index+index+1-i]) {
                return 0;
            }
        }
        println!("same_row, index = {}", index);
        return (index+1) as i64;
    } else if index*2 == map.len()-1 {
        for i in 1..index+1 {
            if !same_row(&map[i], &map[index+index+1-i]) {
                return 0;
            }
        }
        println!("same_row, index = {}", index);
        return (index+1) as i64;
    } else {
        //println!("hej2, index = {}", index);
        for i in index..map.len() {
            if !same_row(&map[i], &map[index-1-(i-index)]) {
                return 0;
            }
        }
        println!("same_row, index = {}", index);
        return index as i64;
    }
    //(index+1) as i64
}


fn same_row(row_1: &Vec<char>, row_2: &Vec<char>) -> bool {
    for i in 0..row_1.len() {
        if row_1[i] != row_2[i] { //kanske funkar
            return false;
        }
    }
    true
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
