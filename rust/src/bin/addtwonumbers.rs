use std::io;

fn main() {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("");
    print!("{}", i.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).into_iter().sum::<i128>());
}