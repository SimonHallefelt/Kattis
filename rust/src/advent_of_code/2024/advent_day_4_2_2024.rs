use std::fs::read_to_string;

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_4.txt"; // 1936
    let map = read_input(file_path);

    let mut tot = 0;
    for i in 1..map.len()-1 {
        for j in 1..map.get(i).unwrap().len()-1 {
            if *map.get(i).unwrap().get(j).unwrap() == 'A' {
                tot += xmas(&map, i, j);
            }
        }
    }

    println!("{}", tot)
}

fn xmas(map: &Vec<Vec<char>>, y: usize, x: usize) -> i64 {
    let b_r = *map.get((y as i32 +  1) as usize).unwrap().get((x as i32 +  1) as usize).unwrap();
    let b_l = *map.get((y as i32 +  1) as usize).unwrap().get((x as i32 + -1) as usize).unwrap();
    let t_r = *map.get((y as i32 + -1) as usize).unwrap().get((x as i32 +  1) as usize).unwrap();
    let t_l = *map.get((y as i32 + -1) as usize).unwrap().get((x as i32 + -1) as usize).unwrap();
    if (t_r == 'M' && b_l == 'S') || (t_r == 'S' && b_l == 'M') {
        if (t_l == 'M' && b_r == 'S') || (t_l == 'S' && b_r == 'M') {
            return 1;
        }
    }
    
    return 0;
}

fn read_input(path: &str) -> Vec<Vec<char>> {
    let content = read_to_string(path).expect("could not read");
    content.trim().split("\n").map(|a| a.trim().chars().collect()).collect()
}