use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let _: usize = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().parse().unwrap();
    let (mut min, mut max, mut g) = (0usize, 10usize.pow(9), 0usize);
    loop {
        if min + 67108864 < max {
            g = min + 67108864;
        } else {
            g = (max + min) / 2;
        }
        println!("{}", g);

        let response = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().to_string();
        if response == "correct" {
            break;
        } else if response == "higher" {
            min = g+1;
        } else {
            max = g-1;
        }
    }
}