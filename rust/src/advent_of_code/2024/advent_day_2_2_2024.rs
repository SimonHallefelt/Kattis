use std::fs;

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_2.txt"; // 400
    let reports = read_input(file_path);

    let mut reports_diff = Vec::new();
    for r in reports {
        let mut v = Vec::new();
        for j in 0..r.len() {
            let mut vv = Vec::new();
            for i in 0..r.len()-1 {
                if i == j {
                    continue;
                }
                else if i+1 == j && j != r.len()-1 {
                    let temp = r.get(i).unwrap() - r.get(i+2).unwrap();
                    vv.push(temp);
                    continue;
                } else if i+1 == j && j == r.len()-1 {
                    continue;
                }
                let temp = r.get(i).unwrap() - r.get(i+1).unwrap();
                vv.push(temp);
            }
            v.push(vv);
        }
        let mut vv = Vec::new();
        for i in 0..r.len()-1 {
            let temp = r.get(i).unwrap() - r.get(i+1).unwrap();
            vv.push(temp);
        }
        v.push(vv);
        reports_diff.push(v);
    }

    let mut safe = 0;
    for rr in reports_diff {
        for r in rr {
            let min = *r.iter().min().unwrap();
            let max = *r.iter().max().unwrap();
            if min * max > 0 && min.abs() <= 3 && max.abs() <= 3 {
                safe += 1;
                break;
            }
        }
    }

    println!("{}", safe)
}

fn read_input(path: &str) -> Vec<Vec<i64>> {
    let input = fs::read_to_string(path).expect("could not read string");
    input.trim().split("\n").map(|a| a.trim().split(" ").map(|b| b.parse().unwrap()).collect()).collect()
}