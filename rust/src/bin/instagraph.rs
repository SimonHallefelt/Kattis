use std::{collections::{HashMap, HashSet}, io};

fn main() {
    let mut input = read_line_stdin();
    let n = input[0];
    let m = input[1];

    if m == 0 {
        println!("1 0");
        return;
    }

    let mut hm: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in 1..n+1 {
        hm.insert(i, HashSet::new());
    }
    for _ in 0..m {
        input = read_line_stdin();
        let u = input[0];
        let v = input[1];
        if hm.get(&u).unwrap().contains(&v) {
            hm.get_mut(&u).unwrap().remove(&v);
        }else {
            hm.get_mut(&v).unwrap().insert(u);
        }
    }

    let mut vec = Vec::with_capacity(hm.len());
    for (k, v) in hm {
        vec.push((k, v.len()))
    }

    vec.sort_by_key(|item| item.1);
    let mut b = *vec.last().unwrap();
    for v in vec.iter().rev() {
        if b.1 != v.1 {
            break;
        } else if b.0 > v.0 {
            b.0 = v.0
        }
    }
    println!("{} {}", b.0, b.1)
}

fn read_line_stdin() -> Vec<i32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();
    input.split_whitespace().map(|f| f.parse().unwrap()).collect()
}