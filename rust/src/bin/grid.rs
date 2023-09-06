use std::{io, collections::{VecDeque, HashSet}};

fn main(){
    let input = read_line();
    let n = input[0].parse::<i64>().unwrap();
    let m = input[1].parse::<i64>().unwrap();
    let mut nodes: Vec<Vec<i64>> = Vec::new();
    for i in 0..n{
        nodes.push(Vec::new());
        let input = read_line();
        let a = input[0].clone();
        let b: Vec<char> = a.chars().collect();
        for j in 0..m{
            let c = b[j as usize].to_string().parse::<i64>().unwrap();
            nodes[i as usize].push(c);
        } 
    }
    println!("{}", bfs((0, 0), (n-1, m-1), nodes));
}

fn bfs(start: (i64, i64), end: (i64, i64), nodes: Vec<Vec<i64>>) -> i64{
    let mut que: VecDeque<(i64, i64, i64)> = VecDeque::new();
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut node: (i64, i64, i64) = (0, 0, 0);
    que.push_back((start.0, start.1, 0));
    visited.insert((node.0, node.1));
    let mut count = -1;
    while !que.is_empty() {
        node = que.pop_front().unwrap();
        if node.0 == end.0 && node.1 == end.1 {
            count = node.2;
            break;
        }
        if node.0 - nodes[node.0 as usize][node.1 as usize] >= 0 {
            let temp = (node.0 - nodes[node.0 as usize][node.1 as usize], node.1);
            if !visited.contains(&temp){
                que.push_back((temp.0, temp.1, node.2+1));
                visited.insert((temp.0, temp.1));
            }
        }
        if node.0 + nodes[node.0 as usize][node.1 as usize] < nodes.len() as i64 {
            let temp = (node.0 + nodes[node.0 as usize][node.1 as usize], node.1);
            if !visited.contains(&temp){
                que.push_back((temp.0, temp.1, node.2+1));
                visited.insert((temp.0, temp.1));
            }
        }
        if node.1 - nodes[node.0 as usize][node.1 as usize] >= 0 {
            let temp = (node.0, node.1 - nodes[node.0 as usize][node.1 as usize]);
            if !visited.contains(&temp){
                que.push_back((temp.0, temp.1, node.2+1));
                visited.insert((temp.0, temp.1));
            }
        }
        if node.1 + nodes[node.0 as usize][node.1 as usize] < nodes[node.0 as usize].len() as i64 {
            let temp = (node.0, node.1 + nodes[node.0 as usize][node.1 as usize]);
            if !visited.contains(&temp){
                que.push_back((temp.0, temp.1, node.2+1));
                visited.insert((temp.0, temp.1));
            }
        }
    }
    count
}

fn read_line() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().split_whitespace().map(|s| s.to_string()).collect()
}