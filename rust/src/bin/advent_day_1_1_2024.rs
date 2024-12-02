use std::fs;

// wrong (to small): 1255306 

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_1.txt"; //1646452
    let (mut left, mut right) = read_line(file_path);
    println!("left len: {:?}", left.len());
    println!("right len: {:?}", right.len());

    left.sort();
    right.sort();

    let mut diff = 0;
    for i in 0..left.len() {
        let l = *left.get(i).unwrap();
        let r = *right.get(i).unwrap();
        diff += (l-r).abs()
    }

    println!("{}", diff);
}

fn read_line(file_path: &str) -> (Vec<i64>, Vec<i64>) {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    
    let mut left = Vec::new();
    let mut right = Vec::new();

    for c in contents.trim().split("\n") {
        let a: Vec<i64> = c.trim().split(" ").filter(|i| !i.is_empty()).map(|i| i.parse::<i64>().unwrap()).collect();
        left.push(*a.get(0).unwrap());
        right.push(*a.get(1).unwrap());
        println!("{:?}, {:?}, {:?}", c, left.last().unwrap(), right.last().unwrap());
    }

    (left, right)
}