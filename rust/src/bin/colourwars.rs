use std::{io, collections::HashMap};

fn main(){
    let n = read_line()[0];
    let mut input = read_line();
    let mut map = HashMap::new();
    for i in input {
        let mut j = i +1;
        if map.contains_key(&j) {
            map.insert(j, map.get(&j).unwrap()+1);
        }else {
            map.insert(j, 1);
        }
    }
    let mut total = 0;
    for (k, v) in map {
        let j = (v/k);
        if v % k == 0 {
            total += (j) * k
        }else {
            total += (j+1) * k
        }
    }
    println!("{}", total);
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}