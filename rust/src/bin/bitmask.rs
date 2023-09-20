use std::{io, collections::{HashMap, HashSet}};

fn main(){
    let s = read_line();
    let input = read_line();
    let mut map = HashMap::new();
    let mut keys = HashSet::new();

    println!("{}", dp(input, s, &mut map, &mut keys));
}

fn dp(price: Vec<i64>, str: Vec<i64>, map: &mut HashMap<i64, i64>, keys: &mut HashSet<i64>) -> i64 {
    
    for s in str {
        for k in keys.clone() {

        }
    }
    0
}


fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}