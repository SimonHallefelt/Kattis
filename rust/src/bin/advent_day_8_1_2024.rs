use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_8.txt"; // 299
    let map = read_input(file_path);

    let mut char_pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let h = map.len();
    let w = map.get(0).unwrap().len();
    for i in 0..h {
        for j in 0..w {
            let c = *map.get(i).unwrap().get(j).unwrap();
            if c == '.' {
                continue;
            }
            if !char_pos.contains_key(&c) {
                char_pos.insert(c, vec![(i as i32, j as i32)]);
            } else {
                char_pos.get_mut(&c).unwrap().push((i as i32, j as i32));
            }
        }
    }

    let mut antinode: HashSet<(i32, i32)> = HashSet::new();
    for cp in char_pos {
        for i in 0..cp.1.len()-1 {
            for j in i+1..cp.1.len() {
                let p1 = cp.1.get(i).unwrap();
                let p2 = cp.1.get(j).unwrap();
                let np1 = (p1.0 + (p1.0-p2.0), p1.1 + (p1.1-p2.1));
                let np2 = (p2.0 + (p2.0-p1.0), p2.1 + (p2.1-p1.1));
                dbg!("{}{}{}{}", p1, p2, np1, np2);
                if (0..h as i32).contains(&np1.0) && (0..w as i32).contains(&np1.1) {
                    antinode.insert(np1);
                }
                if (0..h as i32).contains(&np2.0) && (0..w as i32).contains(&np2.1) {
                    antinode.insert(np2);
                }
            }
        }
    }

    println!("{}", antinode.len())
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let content = read_to_string(path).expect("could not read file");
    content.trim().split("\n").map(|a| a.trim().chars().collect()).collect()
}