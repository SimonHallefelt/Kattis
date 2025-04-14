use std::{collections::HashMap, fs::read_to_string};

fn main() {println!("{}", run("src\\advent_of_code\\2024\\data\\day_11.txt", 75));}

fn run(file_path: &str, d: i64) -> u64 {
    let stones = read_input(file_path);

    let mut h_map = HashMap::new();
    let mut tot = 0;
    for s in stones {
        tot += blink(&s, &mut h_map, d);
    }

    println!("{}", h_map.len());
    tot
}

fn blink(stone: &String, h_map: &mut HashMap<(String, i64), u64>, d: i64) -> u64 {
    if h_map.contains_key(&(stone.clone(), d)) {
        return *h_map.get(&(stone.clone(), d)).unwrap();
    } else if d == 0 {
        return 1;
    }

    let mut tot = 0;
    if stone == "0" {
        let s = "1".to_string();
        tot += blink(&s, h_map, d-1);
    } else if stone.len() % 2 == 0 {
        let temp = stone.split_at(stone.len()/2);
        let temp = (temp.0.trim().to_string(),temp.1.trim().to_string());
        tot += blink(&temp.0, h_map, d-1);
        tot += blink(&temp.1.parse::<u64>().unwrap().to_string(), h_map, d-1);
    } else {
        let s = stone.parse::<u64>().unwrap()*2024;
        tot += blink(&s.to_string(), h_map, d-1);
    }

    h_map.insert((stone.clone(), d), tot);
    tot
}

fn read_input(path: &str) -> Vec<String>{
    let content = read_to_string(path).expect("could nor read file");
    content.trim().split_whitespace().map(|a| a.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        assert_eq!(55312, run("src\\advent_of_code\\2024\\data\\day_11_test.txt", 25))
    }

    #[test]
    fn actual_data() {
        assert_eq!(223894720281135, run("src\\advent_of_code\\2024\\data\\day_11.txt", 75))
    }
}