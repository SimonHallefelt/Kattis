use std::{collections::{VecDeque, HashSet}, io};

fn main(){
    let input = read_line();
    let f = input[0];
    let s = input[1];
    let g = input[2];
    let u = input[3];
    let d = input[4];

    let c = bfs(f, s-1, g-1, u, d);
    if c == -1 {
        println!("use the stairs");
    }else{
        println!("{}", c);
    }
}

fn bfs(f:i64, start: i64, end: i64, u:i64, d:i64) -> i64{
    let mut que: VecDeque<(i64,i64)> = VecDeque::new();
    let mut visited: HashSet<i64> = HashSet::new();
    que.push_back((start, 0));
    visited.insert(start);
    let mut node;
    let mut count = -1;
    while !que.is_empty() {
        node = que.pop_front().unwrap();
        if node.0 == end {
            count = node.1;
            break;
        }
        if node.0 - d >= 0 {
            let i = node.0 - d;
            if !visited.contains(&i){
                que.push_back((i, node.1+1));
                visited.insert(i);
            }
        }
        if node.0 + u < f {
            let i = node.0 + u;
            if !visited.contains(&i){
                que.push_back((i, node.1+1));
                visited.insert(i);
            }
        }
    }
    count
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}