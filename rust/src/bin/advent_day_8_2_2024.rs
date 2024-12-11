use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_8.txt"; // 1032
    let map = read_input(file_path);

    let mut char_pos: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let h = map.len() as i32;
    let w = map.get(0).unwrap().len() as i32;
    for i in 0..h {
        for j in 0..w {
            let c = *map.get(i as usize).unwrap().get(j as usize).unwrap();
            if c == '.' {
                continue;
            }
            if !char_pos.contains_key(&c) {
                char_pos.insert(c, vec![(i, j)]);
            } else {
                char_pos.get_mut(&c).unwrap().push((i, j));
            }
        }
    }

    let mut antinode: HashSet<(i32, i32)> = HashSet::new();
    for cp in char_pos {
        for i in 0..cp.1.len()-1 {
            for j in i+1..cp.1.len() {
                let mut p1 = *cp.1.get(i).unwrap();
                let mut p2 = *cp.1.get(j).unwrap();
                let dif = (p1.0-p2.0, p1.1-p2.1);
                while (0..h).contains(&p1.0) && (0..w).contains(&p1.1) {
                    antinode.insert(p1);
                    p1 = (p1.0 + dif.0, p1.1 + dif.1);
                }
                while (0..h).contains(&p2.0) && (0..w).contains(&p2.1) {
                    antinode.insert(p2);
                    p2 = (p2.0 - dif.0, p2.1 - dif.1);
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