use std::{collections::HashSet, io};

fn main() {
    let input = read_line();
    let input: Vec<&str> = input.split(" ").collect();
    let n: i32 = input[0].parse().unwrap();
    let m: i32 = input[1].parse().unwrap();

    let mut p = Vec::new();
    for _ in 0..m {
        let mut v: Vec<i32> = Vec::new();
        p.push(v);
    }

    let mut set = HashSet::new();
    for _ in 0..m {
        let input = read_line();
        let input: Vec<&str> = input.split(" ").collect();
        let i: i32 = input[0].parse().unwrap();
        let j: i32 = input[1].parse().unwrap();
        p[i as usize].push(j);
        // p[j as usize].push(i);
        set.insert(i);
        set.insert(j);
    }

    for i in 0..m {
        if !set.contains(&i) {
            continue;
        }
        if p[i as usize].is_empty() {
            continue;
        }

        for j in p[i as usize].clone() {
            run(j, &mut p, &mut set, &mut HashSet::new());


        }

    }

}

fn run(i: i32, p: &mut Vec<Vec<i32>>, set: &mut HashSet<i32>, s: &mut HashSet<i32>) -> i32 {
    if p[i as usize].is_empty() {
        return i;
    }

    let v = Vec::new();
    for j in p[i as usize].clone() {
        run(j, &mut p, &mut set, s);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}