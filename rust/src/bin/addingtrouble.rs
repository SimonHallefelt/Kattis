use std::io;
fn main() {
    let mut i = String::new();
    io::stdin().read_line(&mut i).expect("");
    let i: Vec<i128> = i.trim().split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
    if i[0] + i[1] == i[2] {println!("correct!")} else {println!("wrong!")}
}