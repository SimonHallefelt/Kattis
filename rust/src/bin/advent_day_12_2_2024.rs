use std::{collections::{HashMap, HashSet}, fs::read_to_string};

fn main() {run("src\\advent_of_code\\2024\\data\\day_12.txt");}

fn run(path: &str) -> usize {
    let map = read_input(path);
    let h = map.len();
    let w = map.get(0).unwrap().len();

    let mut neighbors = HashMap::new();
    let dirs = vec![(1,0),(0,1),(-1,0),(0,-1)];
    for i in 0..h {
        for j in 0..w {
            let c = map.get(i).unwrap().get(j).unwrap();
            let mut v = Vec::new();
            for d in dirs.iter() {
                let p = (i as i32 + d.0, j as i32 + d.1);
                if (0..h as i32).contains(&p.0) && (0..w as i32).contains(&p.1) {
                    let pp = (p.0 as usize, p.1 as usize);
                    if c == map.get(pp.0).unwrap().get(pp.1).unwrap() {
                        v.push((pp.0 + 2, pp.1 + 2));
                    }
                }
            }
            neighbors.insert((i+2, j+2), v);
        }
    }

    let mut tot = 0;
    let mut visited = HashSet::new();
    for i in 0+2..h+2 {
        for j in 0+2..w+2 {
            let mut area = HashSet::new();
            let mut v = Vec::new();
            if visited.insert((i, j)) && area.insert((i, j)) {
                v.push((i, j));
            }
            while !v.is_empty() {
                let t = neighbors.get(&v.pop().unwrap()).unwrap();
                for tt in t {
                    if area.insert(tt.clone()) {
                        v.push(*tt);
                    }
                }
            }
            visited.extend(&area);


            let mut border = HashSet::new();
            for a in area.clone() {
                for d in dirs.iter() {
                    let p = ((a.0 as i32 + d.0) as usize, (a.1 as i32 + d.1) as usize);
                    if !area.contains(&p) {
                        border.insert(p);
                    }
                }
            }
            let mut n = 0;
            let mut m = 0;

            for b in border.clone() {
                for (k, d) in dirs.clone().iter().enumerate() {
                    let p: (usize, usize) = ((b.0 as i32 + d.0) as usize, (b.1 as i32 + d.1) as usize);
                    if area.contains(&p) {
                        n += 1;
                        let d1 = dirs.get(((k+4)+1)%4).unwrap();
                        let p1 = ((b.0 as i32 + d1.0) as usize, (b.1 as i32 + d1.1) as usize);
                        let pp1 = ((p.0 as i32 + d1.0) as usize, (p.1 as i32 + d1.1) as usize);
                        let d2 = dirs.get(((k+4)-1)%4).unwrap();
                        let p2 = ((b.0 as i32 + d2.0) as usize, (b.1 as i32 + d2.1) as usize);
                        let pp2 = ((p.0 as i32 + d2.0) as usize, (p.1 as i32 + d2.1) as usize);
                        if border.contains(&p1) && area.contains(&pp1) {
                            m += 1;
                        }
                        if border.contains(&p2) && area.contains(&pp2) {
                            m += 1;
                        }
                    }
                }
            }

            tot += area.len() * (n - (m / 2))
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
        assert_eq!(80, run("src\\advent_of_code\\2024\\data\\day_12_test1.txt"));
    }

    #[test]
    fn test_data2() {
        assert_eq!(436, run("src\\advent_of_code\\2024\\data\\day_12_test2.txt"));
    }

    #[test]
    fn test_data3() {
        assert_eq!(1206, run("src\\advent_of_code\\2024\\data\\day_12_test3.txt"));
    }

    #[test]
    fn test_data4() {
        assert_eq!(236, run("src\\advent_of_code\\2024\\data\\day_12_test4.txt"));
    }

    #[test]
    fn test_data5() {
        assert_eq!(368, run("src\\advent_of_code\\2024\\data\\day_12_test5.txt"));
    }

    #[test]
    fn actual_data() {
        assert_eq!(821372, run("src\\advent_of_code\\2024\\data\\day_12.txt"));
    }
}