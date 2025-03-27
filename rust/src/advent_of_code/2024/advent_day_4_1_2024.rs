use std::fs::read_to_string;

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_4.txt"; // 2633
    let map = read_input(file_path);

    let mut tot = 0;
    for i in 0..map.len() {
        for j in 0..map.get(i).unwrap().len() {
            if *map.get(i).unwrap().get(j).unwrap() == 'X' {
                tot += xmas(&map, i as i32, j as i32);
            }
        }
    }

    println!("{}", tot)
}

fn xmas(map: &Vec<Vec<char>>, y: i32, x: i32) -> i64 {
    let modi = vec![(1,0),(-1,0),(1,1),(1,-1),(-1,1),(-1,-1),(0,1),(0,-1)];
    let mut tot = 0;
    for m in modi {
        if y + m.0*3 < 0 || y + m.0*3 >= map.len() as i32 {
            continue;
        }
        if x + m.1*3 < 0 || x + m.1*3 >= map.get(y as usize).unwrap().len() as i32 {
            continue;
        }

        let c1 = map.get((y + m.0*1) as usize).unwrap().get((x + m.1*1) as usize).unwrap();
        let c2 = map.get((y + m.0*2) as usize).unwrap().get((x + m.1*2) as usize).unwrap();
        let c3 = map.get((y + m.0*3) as usize).unwrap().get((x + m.1*3) as usize).unwrap();
        if *c1 == 'M' && *c2 == 'A' && *c3 == 'S' {
            tot += 1;
        }
    }
    tot
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let content = read_to_string(path).expect("could not read");
    let map = content.trim().split("\n").collect::<Vec<&str>>();
    map.iter().map(|a| a.trim().chars().collect()).collect()
}