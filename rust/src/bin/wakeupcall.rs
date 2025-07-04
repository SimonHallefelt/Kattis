use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().skip(1).take(2);
    let sum1: i64 = lines.nth(0).unwrap().unwrap().trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).sum();
    let sum2: i64 = lines.nth(0).unwrap().unwrap().trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).sum();
    if sum1 == sum2 {
        println!("Oh no")
    } else if sum1 > sum2 {
        println!("Button 1")
    }
    else  {
        println!("Button 2")
    }
}