use std::{fs, collections::HashMap};

fn main(){
    let mut total: i64 = 0;
    
    //let file_path = "src\\advent_of_code\\2023\\data\\day_10_test3.txt"; //8
    //let file_path = "src\\advent_of_code\\2023\\data\\day_10_test4.txt"; //10
    let file_path = "src\\advent_of_code\\2023\\data\\day_10.txt"; //595
    let contents = fs::read_to_string(file_path)
        .expect("\n---\nShould have been able to read the file\n---\n");

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

    let mut map2 = Vec::new();
    for i in 0..map.len()*2+3 {
        map2.push(Vec::new());
        for j in 0..map[0].len()*2+3 {
            if j == 0 || i == 0 {
                map2[i].push(false);
            } else if j == map[0].len()*2+2 || i == map.len()*2+2 {
                map2[i].push(false);
            } else {
                map2[i].push(true);
            }
        }
    }

    walk(&map, &mut map2, start, map[start.0][start.1].1);
    bfs(&mut map2, (1, 1));

    for i in (2..map2.len()-2).step_by(2) {
        for j in (2..map2[0].len()-2).step_by(2) {
            if map2[i][j] {
                total += 1;
            }
        }
    }

    println!("total = {}", total);
}


fn bfs(map: &mut Vec<Vec<bool>>, pos: (usize, usize)) {
    let mut to_visit = Vec::new();
    to_visit.push(pos);
    let mut start;
    while !to_visit.is_empty() {
        start = to_visit.pop().unwrap();
        if map[start.0-1][start.1] {
            map[start.0-1][start.1] = false;
            to_visit.push((start.0-1, start.1));
        }
        if map[start.0+1][start.1] {
            map[start.0+1][start.1] = false;
            to_visit.push((start.0+1, start.1));
        }
        if map[start.0][start.1-1] {
            map[start.0][start.1-1] = false;
            to_visit.push((start.0, start.1-1));
        }
        if map[start.0][start.1+1] {
            map[start.0][start.1+1] = false;
            to_visit.push((start.0, start.1+1));
        } 
    }
}


fn walk(map: &Vec<Vec<(char, char)>>, map2: &mut Vec<Vec<bool>>, start: (usize, usize), mut dir: char) {
    let mut next = Vec::new();
    next.push(start);
    let mut pos = (usize::MAX, usize::MAX);
    while start != pos {
        pos = next.pop().unwrap();
        let p = map[pos.0][pos.1];
        if p.0 == dir {dir = p.1}
        else {dir = p.0}

        let temp = go(pos, dir);
        mark(map2, pos, temp.0);
        pos = temp.0;
        dir = temp.1;

        next.push(pos)
    }
}


fn mark(map: &mut Vec<Vec<bool>>, start: (usize, usize), end: (usize, usize)) {
    let a = end.0 as i64 - start.0 as i64;
    let b = end.1 as i64 - start.1 as i64;
    map[start.0*2+(2+a) as usize][start.1*2+(2+b) as usize] = false;
    map[end.0*2+2][end.1*2+2] = false;
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
