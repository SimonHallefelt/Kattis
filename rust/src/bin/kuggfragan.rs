use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let v: Vec<usize> = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, m) = (v[0], v[1]);

    let mut color = vec![0; n];
    let mut neighbors = vec![Vec::new(); n];
    for line in stdin.lock().lines().take(m) {
        let row: Vec<usize> = line.unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (i, j) = (row[0], row[1]);
        neighbors[i].push(j);
        neighbors[j].push(i);
    }
    
    for i in 0..n {
        if color[i] == 0 {
            color[i] = 1;
            if !dfs(i, &mut color, &neighbors) {
                println!("no way");
                return;
            }
        }
    }
    println!("attend here");
}

fn dfs(node: usize, color: &mut Vec<usize>, neighbors: &Vec<Vec<usize>>) -> bool {
    let c = color[node];
    for n in &neighbors[node] {
        if color[*n] == 0 {
            color[*n] = match c {
                1 => 2,
                _ => 1,
            };
            if !dfs(*n, color, neighbors) {
                return false;
            }
        
        } else if color[*n] == c {
            return false;
        }
    }
    true
}