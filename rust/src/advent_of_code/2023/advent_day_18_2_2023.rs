use std::{fs, cmp::{max, min}, collections::{HashMap, HashSet}};


fn main() {
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_18_test.txt"; //952408144115
    let file_path = "src\\advent_of_code\\2023\\data\\day_18.txt"; //
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
    let mut l = i32::MAX;
    let mut r = 0;
    let mut u = i32::MAX;
    let mut d = 0;
    let directions = Vec::from([(0,1), (-1,0), (0,-1), (1,0)]);
    let mut pos = (0,0);
    for i in instructions.clone() {
        let temp: Vec<char> = i[2].trim().chars().filter(|&x| x != '(').filter(|&x| x != ')').collect();
        let dir = directions[temp[temp.len()-1].to_digit(10).unwrap() as usize];
        let mut dis = 0;
        for j in 1..temp.len()-1 {
            dis = dis*16 + temp[j].to_digit(16).unwrap();
        }

        pos = (pos.0 + dir.0*dis as i32, pos.1 + dir.1*dis as i32);
        l = min(pos.1, l);
        r = max(pos.1, r);
        u = min(pos.0, u);
        d = max(pos.0, d);
    }
    println!("l = {}, r = {}, u = {}, d = {}", l , r, u, d);


    let mut total = 0;
    let mut pos = (u.abs(), l.abs());
    let mut hitt_edge = 1;
/*     for i in instructions {
        let temp: Vec<char> = i[2].trim().chars().filter(|&x| x != '(').filter(|&x| x != ')').collect();
        println!("temp = {:?}", temp);
        let dir = directions[temp[temp.len()-1].to_digit(10).unwrap() as usize];
        let mut dis = 0;
        for j in 1..temp.len()-1 {
            dis = dis*16 + temp[j].to_digit(16).unwrap()
        }

        let new_pos = (pos.0 + dir.0*dis as i64, pos.1 + dir.1*dis as i64);

        if pos.1 == new_pos.1 {
            total += dis as i64 * (new_pos.0 - pos.0);
        } else if pos.1 > new_pos.1 {
            total -= (dis - 1) as i64 * pos.0;
        } else {
            total += (dis - 1) as i64 * pos.0;
        }

        pos = new_pos;
    } */
    println!("walked map");

    let mut hs: HashSet<(i32, i32)> = HashSet::new();
    for i in instructions {
        let temp: Vec<char> = i[2].trim().chars().filter(|&x| x != '(').filter(|&x| x != ')').collect();
        //println!("temp = {:?}", temp);
        let dir = directions[temp[temp.len()-1].to_digit(10).unwrap() as usize];
        let mut dis = 0;
        for j in 1..temp.len()-1 {
            dis = dis*16 + temp[j].to_digit(16).unwrap()
        }


        for _ in 0..dis {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            hs.insert(pos);
        }
    }
    //println!("hs start = {:?}", hs.get(&l.abs()));
    println!("walked map");

    total += hs.len() as i64;
    total += bfs(hs.clone(), (u.abs() as i32 -1, l.abs() as i32 +1));
    println!("bfs done");

    total
}


fn bfs(edge: HashSet<(i32, i32)>, start: (i32,i32)) -> i64 {
    let mut total = 0;
    let mut old = HashSet::new();
    let mut current = HashSet::new();
    let mut new = HashSet::new();

    let directions = Vec::from([(0,1), (-1,0), (0,-1), (1,0)]);
    let mut steps = 10000000;
    current.insert(start);
    while !current.is_empty() {
        for p in &current {
            for dir in &directions {
                let new_pos = (p.0+dir.0, p.1+dir.1);
                if old.contains(&new_pos) {continue;}
                if current.contains(&new_pos) {continue;}
                if edge.contains(&new_pos) {continue;}
                if new_pos.0 < 0 || new_pos.1 < 0 {println!("---Fel---"); return 0;}
                new.insert(new_pos);
            }
        }

        total += current.len() as i64;
        if total > steps {
            println!("total = {}", total);
            steps += 10000000;
        }
        old = current;
        current = new;
        new = HashSet::new();
    }

    total
}


fn fix_input(line: &str) -> Vec<String> {
    line.trim().split(" ").map(|x| x.to_string()).collect()
}
