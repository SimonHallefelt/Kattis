use std::fs::read_to_string;

fn main() {run("src\\advent_of_code\\2024\\data\\day_11.txt");}

fn run(file_path: &str) -> usize {
    let mut stones = read_input(file_path);

    for _ in 0..25 {
        let l = stones.len();
        for i in 0..l {
            if stones.get(i).unwrap() == "0" {
                *stones.get_mut(i).unwrap() = "1".to_string();
            } else if stones.get_mut(i).unwrap().len() % 2 == 0 {
                let temp = stones.get(i).unwrap()
                    .split_at(stones.get(i).unwrap().len()/2);
                let temp = (temp.0.trim().to_string(),temp.1.trim().to_string());
                *stones.get_mut(i).unwrap() = temp.0;
                stones.push(temp.1.parse::<u64>().unwrap().to_string());
            } else {
                let s = stones.get(i).unwrap().parse::<u64>().unwrap()*2024;
                *stones.get_mut(i).unwrap() = s.to_string();
            }
        }
    }

    stones.len()
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
        assert_eq!(55312, run("src\\advent_of_code\\2024\\data\\day_11_test.txt"))
    }

    #[test]
    fn actual_data() {
        assert_eq!(188902, run("src\\advent_of_code\\2024\\data\\day_11.txt"))
    }
}