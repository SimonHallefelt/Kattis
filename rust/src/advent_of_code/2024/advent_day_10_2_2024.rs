use std::fs::read_to_string;

fn main() {run("src\\advent_of_code\\2024\\data\\day_10.txt");}

fn run(file_path: &str) -> u32 {
    let (map, zeros) = read_data(file_path);

    let mut tot = 0;
    for z in zeros {
        find_paths(&map, z.0+1, z.1, 0, &mut tot);
        find_paths(&map, z.0-1, z.1, 0, &mut tot);
        find_paths(&map, z.0, z.1+1, 0, &mut tot);
        find_paths(&map, z.0, z.1-1, 0, &mut tot);
    }

    tot
}

fn find_paths(map: &Vec<Vec<u32>>, y: i32, x: i32, current: u32, paths: &mut u32) {
    if !(0..map.len()).contains(&(y as usize)) || !(0..map.get(y as usize).unwrap().len()).contains(&(x as usize)) {
        return;
    } 
    let v = *map.get(y as usize).unwrap().get(x as usize).unwrap();
    if v == current +1 {
        if v == 9 {
            *paths += 1;
        } else {
            find_paths(map, y+1, x, v, paths);
            find_paths(map, y-1, x, v, paths);
            find_paths(map, y, x+1, v, paths);
            find_paths(map, y, x-1, v, paths);
        }
    }
}

fn read_data(path: &str) -> (Vec<Vec<u32>>, Vec<(i32, i32)>) {
    let content = read_to_string(path).expect("could not read file");
    let map:Vec<Vec<u32>> = content.trim().split("\n")
        .map(|a| a.trim().chars()
            .map(|b| b.to_digit(10).unwrap()).collect()
        ).collect();
    let zeros = map.iter().enumerate()
        .map(|a| (*a.1).iter().enumerate()
            .filter(|b| *b.1 == 0).map(|c| (a.0 as i32, c.0 as i32))
            .collect::<Vec<(i32, i32)>>())
        .flatten().collect();
    (map, zeros)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_data() {
        assert_eq!(81, run("src\\advent_of_code\\2024\\data\\day_10_test.txt"))
    }

    #[test]
    fn actual_data() {
        assert_eq!(1925, run("src\\advent_of_code\\2024\\data\\day_10.txt"))
    }
}