use std::{io, collections::{HashSet, HashMap}};

fn main() {
    let input = read_line();
    let start_char = input.chars().collect::<Vec<char>>()[input.len()-1];
    let mut possible = Vec::new();
    let mut gotten = HashMap::new();
    for _ in 0..read_line().parse::<i64>().unwrap() {
        let input = read_line();
        let chars = input.chars().collect::<Vec<char>>();
        if chars[0] == start_char {
            possible.push((chars[0], chars[chars.len()-1], input).clone());
        }
        let temp = gotten.get(&chars[0]).unwrap_or(&0);
        gotten.insert(chars[0], temp.clone()+1);
    }
    if possible.len() == 0 {
        println!("?");
        return;
    }
    for p in possible.clone() {
        let mut i = 0;
        if p.0 == p.1 {
            i = 1;
        }
        if *gotten.get(&p.1).unwrap_or(&0) -i < 1 {
            println!("{}!", p.2);
            return;
        }
    }
    println!("{}", possible[0].2);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}