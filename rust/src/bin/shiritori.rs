use std::{io, collections::HashSet};

fn main(){
    let input = read_line();
    let n = input.parse::<i64>().unwrap();
    let mut set = HashSet::new();
    let mut last = read_line();
    set.insert(last.clone());
    for i in 0..n-1 {
        let input = read_line();
        if set.contains(&input) || input.chars().nth(0).unwrap() != last.chars().nth(last.len()-1).unwrap() {
            let player = (i+1)%2+1;
            println!("Player {} lost", player);
            return;
        }
        set.insert(input.clone());
        last = input;
    }
    println!("Fair Game");
}

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string()
}