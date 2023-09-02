use std::io;

fn main() {
    let n = readline().parse::<i128>().unwrap();
    let input = readline();
    let input: Vec<i128> = input.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
    let mut sum = 1;
    for i in 1..n {
        if input[i as usize] > input[(i-1) as usize] {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}