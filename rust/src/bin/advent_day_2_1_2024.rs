use std::fs;



fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_2.txt"; //334
    let reports = read_input(file_path);

    let mut reports_diff = Vec::new();
    for r in reports {
        let mut v = Vec::new();
        for i in 0..r.len()-1 {
            v.push(r.get(i).unwrap() - r.get(i+1).unwrap());
        }
        reports_diff.push(v);
    }

    let mut safe = 0;
    for r in reports_diff {
        let min = *r.iter().min().unwrap();
        let max = *r.iter().max().unwrap();
        if min * max > 0 && min.abs() <= 3 && max.abs() <= 3 {
            safe += 1;
        }
    }

    println!("{}", safe)
}

fn read_input(path: &str) -> Vec<Vec<i64>> {
    let input = fs::read_to_string(path).expect("could not read string");
    let mut reports = Vec::new();
    for r in input.split("\n") {
        reports.push(r.trim().split(" ").map(|a| a.parse::<i64>().unwrap()).collect());
    }
    reports
}