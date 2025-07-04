use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let n = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().parse().unwrap();
    let mut strength = Vec::with_capacity(n);
    let mut strength_gained = 0;
    for line in stdin.lock().lines().take(n) {
        let row = line.expect("cant read line");
        let temp: Vec<usize> = row.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        if temp[0] < strength_gained {
            strength.push(0);
        } else {
            strength.push(temp[0]-strength_gained); 
        }
        strength_gained += temp[1];
    }
    let mut monster = vec![(strength[0], 1)];
    for i in 1..n {
        let s = strength[i];
        if s <= monster.last().unwrap().0 {
            monster.last_mut().unwrap().1 += 1;
        } else {
            monster.push((s, monster.last().unwrap().1 + 1));
        }
    }

    let q = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().parse().unwrap();
    let mut start_strength;
    for line in stdin.lock().lines().take(q) {
        let row = line.expect("could not read query");
        start_strength = row.trim().parse().unwrap();

        println!("{}", binary_search(start_strength, &monster));
    }
}

fn binary_search(s: usize, monster: &Vec<(usize,usize)>) -> usize {
    if s < monster.first().unwrap().0 {return 0;}
    if s >= monster.last().unwrap().0 {return monster.last().unwrap().1;}
    
    let mut min = 0;
    let mut max = monster.len()-1;
    let mut mid;
    loop {
        mid = (max + min) / 2;
        let ms = monster[mid].0;
        if s < ms {
            max = mid;
            continue;
        } else if s < monster[mid+1].0 {
            break;
        } else {
            min = mid + 1;
            continue;
        }
    }
    monster[mid].1
}