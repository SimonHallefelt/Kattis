use std::{collections::{HashMap, HashSet}, io};

fn main() {
    let input = read_line();
    let input: Vec<&str> = input.split(" ").collect();
    let n: i32 = input[0].parse().unwrap();
    let m: i32 = input[1].parse().unwrap();

    let mut map: HashMap<String, Vec<i32>> = HashMap::new();
    for _ in 0..n {
        let input = read_line();
        let input: Vec<&str> = input.split(" ").collect();
        let i: i32 = input[0].parse().unwrap();
        let s  = input[1].to_string();
        if map.contains_key(&s) {
            let mut temp = map.get_mut(&s).unwrap().clone();
            temp.push(i);
            map.insert(s, temp);
        } else {
            map.insert(s, vec![i]);
        }
    }

    let mut set = HashSet::new();
    set.insert(0);
    for v in map.values() {
        let mut newset = HashSet::new();
        for i in set.iter() {
            for j in v {
                if i + j <= m {
                    newset.insert(i+j);
                }
            }
        }
        for i in newset {
            set.insert(i);
        }
    }
    let t = set.iter().max().unwrap();
    println!("{}", t)
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}