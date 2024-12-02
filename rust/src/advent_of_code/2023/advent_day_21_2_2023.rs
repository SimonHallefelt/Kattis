use std::{fs, collections::{HashMap, VecDeque, HashSet}};

//kartan blockar inget viktigt så det blir en perfect kvadrat snurad 45 grader.
fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_21_test.txt"; //
    let file_path = "src\\advent_of_code\\2023\\data\\day_21.txt"; //
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
    println!("hej2, start = {:?}, map = {:?}, map[0] = {:?}", start, map.len(), map[0].len());

    total += run(&map, start);

    println!("total = {}", total);
}


fn run(map: &Vec<Vec<char>>, start: (i32,i32)) -> i64 {
    let mut reach = HashSet::new();
    let dir = Vec::from([(0,1),(0,-1),(1,0),(-1,0)]);
    reach.insert(start);
    let steps = 131+65 +131*3; //26501365;

    for i in 0..steps {
        let mut reach_new = HashSet::new();
        for r in reach {
            for d in &dir {
                let pos = (r.0+d.0, r.1+d.1);
                let mut a_pos = pos;
                if pos.0 < 0 {
                    a_pos.0 = (pos.0%map.len() as i32) + map.len() as i32;
                    a_pos.0 %= map.len() as i32;
                } if pos.1 < 0 {
                    a_pos.1 = (pos.1%map[0].len() as i32) + map[0].len() as i32;
                    a_pos.1 %= map[0].len() as i32;
                } if pos.0 >= map.len() as i32 {
                    a_pos.0 = pos.0%map.len() as i32;
                } if pos.1 >= map[0].len() as i32 {
                    a_pos.1 = pos.1%map[0].len() as i32;
                }
                if map[a_pos.0 as usize][a_pos.1 as usize] != '#' {
                    reach_new.insert((pos.0, pos.1));
                }
            }
        }
        reach = reach_new;
        //println!("reach = {:?}", reach);
        if i % 100 == 0 {
            println!("i = {}, reach.len() = {:?}", i, reach.len());
        }
    }

    println!("reach = {:?}", reach.len());
    println!("26501365 - (131 + 65) = {}", 26501365 - (131 + 65));
    println!("(26501365 - (131 + 65))/131 = {}", (26501365 - (131 + 65))/131);
    println!("((26501365 - (131 + 65))/131)*131 = {}", ((26501365 - (131 + 65))/131)*131);
    let t = (26501365 - (131 + 65))/131;
    //let t = 26501365 - (131 + 65);
    println!("t = {}", t);
    println!("t/3 = {}", t/3);
    println!("(t/3)*3 = {}", (t/3)*3);

    let rl = reach.len() as i128;
    let mut tot = rl;
    let mut i = 1*2;//(t/3)*2;
    tot += rl * i;
    while i != 0 {
        tot += rl * i*2;
        i -= 1;
    }
    println!("rl*9 = {}", rl*9);    //301464 är 300451
    println!("tot = {}", tot);  //räknas kanten 2 gånger?

    //466023128099869 wrong
    //609262351150744 wrong
    //104553973424426 wrong
    //1045539734244261026 to high
    //10455397342442610264 to high
    //94098568980517969816 to high
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
