use std::{collections::HashMap, io};

fn main(){
    let n = read();
    let values = read_vec();
    let mut h: HashMap<i64, i64> = HashMap::new();

    for i in 0..n {
        let v = *values.get(i as usize).unwrap();
        let diff;
        if v <= i+1 {
            diff = i+1-v;
        } else if v < n + 1 {
            diff = n-(v-(i+1));
        } else {
            continue;
        }
        if h.contains_key(&diff) {
            h.insert(diff, h.get(&diff).unwrap()+1);
        } else {
            h.insert(diff, 1);
        }
    }
    let mut best = (0,0);
    for i in h {
        if i.1 > best.1 {
            best = i;
        } else if i.1 == best.1 && i.0 < best.0 {
            best = i;
        }
    }
    println!("{} {}", best.0, best.1)
}


fn read() -> i64{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.parse::<i64>().unwrap()
}


fn read_vec() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}