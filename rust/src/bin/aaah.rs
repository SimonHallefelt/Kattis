use std::io;

fn main(){
    let s1 = read_input();
    let s2 = read_input();
    if s1.len() >= s2.len() {
        println!("go");
    } else {
        println!("no");
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}