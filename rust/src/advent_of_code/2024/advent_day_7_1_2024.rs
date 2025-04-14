use std::fs::read_to_string;

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_7.txt"; // 4555081946288 
    let input = read_input(file_path);

    let mut tot = 0;
    for mut row in input {
        let num = row.1.remove(0);
        tot += run(row.0, row.1, num);
    }

    println!("{}", tot)
}

fn run(target: i64, mut numbers: Vec<i64>, current: i64) -> i64 {
    if current > target {
        return 0;
    } else if current == target && numbers.is_empty() {
        return target;
    } else if numbers.is_empty() {
        return 0;
    }
    let num = numbers.remove(0);
    if target == run(target, numbers.clone(), current+num) {
        return target;
    } else if target == run(target, numbers, current*num) {
        return target;
    } else {
        return 0;
    }
}

fn read_input(file: &str) -> Vec<(i64, Vec<i64>)> {
    let content = read_to_string(file).expect("could not read file");
    // content.trim().split("\n")
    //     .map(|a| a.trim().replace(":", " ").split_whitespace()
    //     .map(|b| b.trim().parse().unwrap()).collect())
    //     .collect()

    content.trim().split("\n")
        .map(|a| (a.trim().split(": ").nth(0).unwrap().trim().parse().unwrap(), a.trim().split(": ").nth(1).unwrap()
        .split_whitespace()
        .map(|c| c.trim().parse().unwrap()).collect()))
        .collect()
}