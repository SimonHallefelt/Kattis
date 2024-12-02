use std::{fs, collections::HashMap};

fn main(){
    let total: i64;
    
    //let file_path = "src\\advent_of_code\\2023\\data\\day_10_test.txt"; //4
    //let file_path = "src\\advent_of_code\\2023\\data\\day_10_test2.txt"; //8
    let file_path = "src\\advent_of_code\\2023\\data\\day_10.txt"; //6867
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut map = Vec::new();
    let mut start = (usize::MAX, usize::MAX);
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        if c.contains('S') {
            start = (map.len(), find_start(c));
        }
        map.push(input);
    }
    map[start.0][start.1] = fix_start(&map, start);

    total = find_farthest(&map, start, map[start.0 as usize][start.1 as usize].1, 0);

    println!("total = {}", total);
}


fn find_farthest(map: &Vec<Vec<(char, char)>>, start: (usize, usize), mut dir: char, mut steps: i64) -> i64 {
    let mut next = Vec::new();
    next.push(start);
    let mut pos = (usize::MAX, usize::MAX);
    while start != pos {
        pos = next.pop().unwrap();
        let p = map[pos.0][pos.1];
        if p.0 == dir {dir = p.1}
        else {dir = p.0}

        let temp = go(pos, dir);
        pos = temp.0;
        dir = temp.1;

        steps += 1;
        next.push(pos);
    }
    return steps/2
}


fn go(pos: (usize, usize), dir: char) -> ((usize, usize), char) {
    match dir {
        'n' => ((pos.0-1, pos.1), 's'),
        's' => ((pos.0+1, pos.1), 'n'),
        'e' => ((pos.0, pos.1+1), 'w'),
        _ => ((pos.0, pos.1-1), 'e')
    }
}


fn fix_start(map: &Vec<Vec<(char, char)>>, start: (usize, usize)) -> (char, char) {
    let mut value = map[start.0][start.1];
    if start.0+1 < map.len() && (map[start.0+1][start.1].0 == 'n' || map[start.0+1][start.1].1 == 'n') {
        if value.0 == '-' {value.0 = 's';}
        else {value.1 = 's';}
    } 
    if start.0 > 0 && (map[start.0-1][start.1].0 == 's' || map[start.0-1][start.1].1 == 's') {
        if value.0 == '-' {value.0 = 'n';}
        else {value.1 = 'n';}
    } 
    if start.1+1 < map[0].len() && (map[start.0][start.1+1].0 == 'w' || map[start.0][start.1+1].1 == 'w') {
        if value.0 == '-' {value.0 = 'e';}
        else {value.1 = 'e';}
        
    } 
    if start.1 > 0  && (map[start.0][start.1-1].0 == 'e' || map[start.0][start.1-1].1 == 'e') {
        if value.0 == '-' {value.0 = 'w';}
        else {value.1 = 'w';}
    }
    value
}


fn find_start(line: &str) -> usize {
    for c in line.trim().chars().enumerate() {
        if c.1 == 'S' {
            return c.0;
        }
    }
    usize::MAX
}


fn fix_input(line: &str) -> Vec<(char, char)>{
    let symbols = HashMap::from([
        ('-', ('w', 'e')),
        ('|', ('n', 's')),
        ('7', ('w', 's')),
        ('J', ('w', 'n')),
        ('F', ('s', 'e')),
        ('L', ('n', 'e')),
        ('.', ('.', '.')),
        ('S', ('-', '-'))
    ]);
    line.trim().chars().map(|x| *symbols.get(&x).unwrap()).collect()
}
