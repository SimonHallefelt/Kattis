use std::{collections::HashSet, io};

fn main() {
    let input = read_line();
    let n: i32 = input.parse().unwrap();
    let mut set = HashSet::new();
    for _ in 0..n {
        let input = read_line();
        set.insert(input);
    }
    let input = read_line();
    let input = input.split(" ");
    let mut b = true;
    for s in input {
        if !set.contains(s) {
            b = false;
        }
    }

    if b {
        println!("Hi, how do I look today?")
    } else {
        println!("Thore has left the chat")

    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_lowercase().to_string()
}