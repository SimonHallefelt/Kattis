use std::{collections::HashSet, io};

fn main(){
    let input = read_line();
    let n = input[0];
    let m = input[1];
    let mut set = HashSet::new();
    for _ in 0..m {
        let input = read_line();
        let a = input[0];
        set.insert(a);
    }
    
    for i in 0..n {
        if !set.contains(&i) {
            println!("{}", i);
        }
    }
    println!("Mario got {} of the dangerous obstacles.", set.len());
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}