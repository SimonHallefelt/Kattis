use std::{io, collections::{VecDeque, HashSet}};

fn main(){
    let k = read_line()[0].parse::<i64>().unwrap();
    for _ in 0..k{
        let n = read_line()[0].parse::<i64>().unwrap();
        let m = read_line()[0].parse::<i64>().unwrap();
        let mut nodes: Vec<Vec<i64>> = Vec::new();
        for _ in 0..n{
            nodes.push(Vec::new());
        }
        for _ in 0..m{
            let input = read_line();
            let a = input[0].parse::<i64>().unwrap();
            let b = input[1].parse::<i64>().unwrap();
            nodes[a as usize].push(b);
            nodes[b as usize].push(a);
        } 
        println!("{}", bfs(0, n, nodes)-1);
    }
}

fn bfs(start: i64, end: i64, nodes: Vec<Vec<i64>>) -> i64{
    let mut que: VecDeque<i64> = VecDeque::new();
    let mut visited: HashSet<i64> = HashSet::new();
    let mut node;
    let mut count = 0;
    for i in start..end {
        if !visited.contains(&i){
            count += 1;
            que.push_back(i);
        }
        while !que.is_empty() {
            node = que.pop_front().unwrap();
            visited.insert(node);
            if node == end {
                break;
            }
            for j in &nodes[node as usize]{
                if !visited.contains(&j){
                    que.push_back(*j);
                }
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