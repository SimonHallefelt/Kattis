use std::{io, collections::HashMap};

fn main(){
    let input = read_input();
    let mut numbers: Vec<i32> = input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
    let input = read_input();
    let order: Vec<String> = input.chars().map(|x| x.to_string()).collect();
    let mut map: HashMap<String, i32> = HashMap::new();

    numbers.sort();
    map.insert("A".to_string(), numbers[0]);
    map.insert("B".to_string(), numbers[1]);
    map.insert("C".to_string(), numbers[2]);

    println!("{} {} {}", map.get(&order[0]).unwrap(), map.get(&order[1]).unwrap(), map.get(&order[2]).unwrap());
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}