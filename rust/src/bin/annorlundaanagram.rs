use std::{io, collections::HashMap};

fn main() {
    let input = readline();
    let n: i128 = input.len() as i128 / 2;
    let mut map: HashMap<char, i128> = HashMap::new();

    for c in input.chars() {
        if map.contains_key(&c) {
            let count = map.get(&c).unwrap();
            map.insert(c, count + 1);
        } else {
            map.insert(c, 1);
        }
    }

    let mut list: Vec<(char, i128)> = map.iter().map(|(c, i)| (*c, *i)).collect::<Vec<_>>();
    list.sort_by(|a, b| a.1.cmp(&b.1));
    list = list.iter().rev().map(|(c, i)| (*c, *i)).collect::<Vec<_>>();
    run(list, n);
}

fn run(mut list: Vec<(char, i128)>, n: i128) {
    let mut result: String = "".to_string();
    if list.len() <= 1{
        println!("-1");
        return;
    }

    if list[0].1 >= n {
        for _ in 0..n {
            result += &list[0].0.to_string();
        }
        list[0].1 -= n;
        result += &list[1].0.to_string();
        list[1].1 -= 1;
    }

    for (c, i) in list {
        for _ in 0..i {
            result += &c.to_string();
        }
    }

    println!("{}", result);
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}