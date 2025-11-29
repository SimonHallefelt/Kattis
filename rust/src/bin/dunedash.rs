use std::{cmp::Ordering, io};

fn main() {
    let n = *read_line_stdin().first().unwrap() as usize;
    let mut vec = Vec::new();
    for _ in 0..n {
        let input = read_line_stdin();
        vec.push((input[0], input[1]));
    }
    for _ in 0..2 {
        let f = vec[n-1];
        vec.sort_by(|a, b| dist(a, &f)
                .partial_cmp(&dist(b, &f))
                .unwrap_or(Ordering::Equal)
        );
    }
    println!("{}", vec.windows(2).map(|a| dist(&a[0], &a[1])).sum::<f64>())
}

fn dist(a: &(i64, i64), b: &(i64, i64)) -> f64 {
    return (((a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)) as f64).sqrt();
}

fn read_line_stdin() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();
    input.split_whitespace().map(|f| f.parse().unwrap()).collect()
}