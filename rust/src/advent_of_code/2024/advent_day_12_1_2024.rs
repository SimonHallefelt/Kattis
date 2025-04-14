use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {run("src\\advent_of_code\\2024\\data\\day_12.txt");}

fn run(path: &str) -> usize {
    let map = read_input(path);
    let h = map.len();
    let w = map.get(0).unwrap().len();

    let mut neighbors = HashMap::new();
    let dirs = vec![(1,0),(-1,0),(0,1),(0,-1)];
    for i in 0..h {
        for j in 0..w {
            let c = map.get(i).unwrap().get(j).unwrap();
            let mut v = Vec::new();
            for d in dirs.iter() {
                let p = (i as i32 + d.0, j as i32 + d.1);
                if (0..h as i32).contains(&p.0) && (0..w as i32).contains(&p.1) {
                    let pp = (p.0 as usize, p.1 as usize);
                    if c == map.get(pp.0).unwrap().get(pp.1).unwrap() {
                        v.push(pp);
                    }
                }
            }
            neighbors.insert((i, j), v);
        }
    }

    let mut tot = 0;
    let mut visited = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            let mut area = HashSet::new();
            let mut n = 0;
            let mut v = Vec::new();
            if visited.insert((i, j)) && area.insert((i, j)) {
                v.push((i, j));
            }
            while !v.is_empty() {
                let t = neighbors.get(&v.pop().unwrap()).unwrap();
                n += t.len();
                for tt in t {
                    if area.insert(tt.clone()) {
                        v.push(*tt);
                    }
                }
            }
            visited.extend(&area);
            tot += area.len() * (area.len()*4 - n)
        }
    }

    tot 
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let content = read_to_string(path).expect("could not read file");
    content.trim().split("\n").map(|a| a.trim().chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data1() {
        assert_eq!(140, run("src\\advent_of_code\\2024\\data\\day_12_test1.txt"));
    }

    #[test]
    fn test_data2() {
        assert_eq!(772, run("src\\advent_of_code\\2024\\data\\day_12_test2.txt"));
    }

    #[test]
    fn test_data3() {
        assert_eq!(1930, run("src\\advent_of_code\\2024\\data\\day_12_test3.txt"));
    }

    #[test]
    fn actual_data() {
        assert_eq!(1375476, run("src\\advent_of_code\\2024\\data\\day_12.txt"));
    }
}