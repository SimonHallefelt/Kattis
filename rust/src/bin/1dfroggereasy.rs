use std::{io, collections::HashSet};

fn main(){
    let input: String = read_line();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let n = input[0].to_string().parse::<i128>().unwrap();
    let start = input[1].to_string().parse::<i128>().unwrap() -1;
    let target = input[2].to_string().parse::<i128>().unwrap();

    let input: String = read_line();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let mut v: Vec<i128> = Vec::new();
    for i in 0..n {
        v.push(input[i as usize].to_string().parse::<i128>().unwrap());
    }
    play(n, start, target, v);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn play(n: i128, start: i128, target: i128, mut v: Vec<i128>){
    let mut set: HashSet<i128> = HashSet::new();
    let mut i = start;
    let mut steps = 0;

    set.insert(i);
    while v[i as usize] != target{
        i += v[i as usize];
        steps += 1;
        if set.contains(&i) || i < 0 || i >= n{
            break;
        }
        set.insert(i);
    }

    if i >= n {
        println!("right");
    }else if i < 0 {
        println!("left");
    }else if v[i as usize] == target {
        println!("magic");
    }else {
        println!("cycle");
    }
    println!("{}", steps);
}