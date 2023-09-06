use std::{io, collections::{VecDeque, HashSet}};

fn main(){
    let input = read_line();
    let n = input[0];
    let m = input[1];
    let mut nodes = vec![Vec::new(); m as usize];
    for _ in 0..m{
        let input = read_line();
        let a = input[0];
        let b = input[1];
        nodes[a as usize].push(b);
        nodes[b as usize].push(a);
    }
}

fn bfs(start: i64, end: i64, nodes: Vec<Vec<i64>>) -> Vec<i64>{
    let mut que: VecDeque<i64> = VecDeque::new();
    let mut visited: HashSet<i64> = HashSet::new();
    let mut node;
    let mut count = 0;
    let mut path = Vec::new();
    que.push_back(start);
    visited.insert(start);
    while !que.is_empty() {
        node = que.pop_front().unwrap();
        visited.insert(node);
        if node == end {
            break;
        }
        for i in &nodes[node.clone() as usize]{
            if !visited.contains(i){
                que.push_back(*i);
                visited.insert(*i);
            }
        }
    }
    path
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}