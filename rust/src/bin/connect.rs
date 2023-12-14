use std::io;

fn main(){
    let input = read_line();
    let A1 = read_line();
    let A2 = read_line();
    let B1 = read_line();
    let B2 = read_line();

    let tot = (A1[0]-A2[0]).abs()+(A1[1]-A2[1]).abs();

    let x = A1[0] as f64 -A2[0] as f64 / A1[1] as f64 - A2[1] as f64;
}

fn read_line() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}