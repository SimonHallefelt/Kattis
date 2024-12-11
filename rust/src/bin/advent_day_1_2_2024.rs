use std::fs;

// wrong (to small): 1255306 

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_1.txt"; //23609874
    let (mut left, mut right) = read_line(file_path);
    println!("left len: {:?}", left.len());
    println!("right len: {:?}", right.len());

    left.sort();
    right.sort();

    let mut diff = 0;
    for i in left {
        let t: Vec<i64> = right.iter().filter(|v| **v == i).map(|v| *v).collect();
        diff += i * t.len() as i64;
    }

    println!("{}", diff);
}

fn read_line(file_path: &str) -> (Vec<i64>, Vec<i64>) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    contents.trim()
        .split("\n")
        .map(|a| a.trim().split_whitespace().map(|b| b.parse().unwrap()).collect())
        .collect::<Vec<Vec<i64>>>()
        .iter()
        .map(|a| (a.first().unwrap(), a.last().unwrap()))
        .collect()
}