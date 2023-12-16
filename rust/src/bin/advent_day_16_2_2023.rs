use std::{fs, collections::HashSet, cmp::max};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_16_test.txt"; //51
    let file_path = "src\\advent_of_code\\2023\\data\\day_16.txt"; //8225
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = Vec::new();
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        map.push(input);
    }

    let mut maxx = 0;
    let dir = Vec::from([(0,1), (0,-1), (1,0), (-1,0)]);
    for i in 0..map.len() {
        maxx = max(maxx, run(&map, ((i as i64, 0), dir[0])));
        maxx = max(maxx, run(&map, ((i as i64, map[0].len() as i64 -1), dir[1])));
    }
    println!("halfway, max = {}", maxx);
    for i in 0..map[0].len() {
        maxx = max(maxx, run(&map, ((0, i as i64), dir[2])));
        maxx = max(maxx, run(&map, ((map.len() as i64 -1, i as i64), dir[3])));
    }
    total += maxx;

    println!("total = {}", total);
}


fn run(map: &Vec<Vec<char>>, start: ((i64,i64),(i64,i64))) -> i64 {
    let mut beams = Vec::new();
    let mut hitt = Vec::new();
    let dir = Vec::from([(0,1), (0,-1), (1,0), (-1,0)]);
    let mut hs = HashSet::new();
    for i in 0..map.len() {
        hitt.push(Vec::new());
        for _ in 0..map[i].len() {
            hitt[i].push(false);
        }
    }
    beams.push(start);

    while beams.len() != 0 {
        let beam = beams.pop().unwrap();
        if !hs.insert(beam) {continue;}
        if beam.0.0 < 0 || beam.0.1 < 0 {continue;}
        if beam.0.0 >= map.len() as i64 || beam.0.1 >= map[0].len() as i64 {continue;}
        hitt[beam.0.0 as usize][beam.0.1 as usize] = true;

        match map[beam.0.0 as usize][beam.0.1 as usize] {
            '|' => {
                if beam.1.0 == 0 {
                    beams.push(((beam.0.0+1, beam.0.1),dir[2]));
                    beams.push(((beam.0.0-1, beam.0.1),dir[3]));
                } else {
                    beams.push(((beam.0.0+beam.1.0, beam.0.1), beam.1));
                }
            },
            '-' => {
                if beam.1.1 == 0 {
                    beams.push(((beam.0.0, beam.0.1+1),dir[0]));
                    beams.push(((beam.0.0, beam.0.1-1),dir[1]));
                } else {
                    beams.push(((beam.0.0, beam.0.1+beam.1.1), beam.1));
                }
            },
            '/' => {
                let new_dir = (beam.1.1*-1, beam.1.0*-1);
                beams.push(((beam.0.0+new_dir.0, beam.0.1+new_dir.1), new_dir));
            },
            '\\' => { //kanske funkar?
                let new_dir = (beam.1.1, beam.1.0);
                beams.push(((beam.0.0+new_dir.0, beam.0.1+new_dir.1), new_dir));
            },
            _ => {beams.push(((beam.0.0+beam.1.0, beam.0.1+beam.1.1), beam.1))}
        }
    }

    hitt.iter().map(|x| x.iter().filter(|&n| *n == true).count() as i64).sum()
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
