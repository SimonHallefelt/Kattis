use std::{io, collections::{VecDeque, HashSet}};

fn main(){
    let input = read_line();
    let n = input[0]+1;
    let m = input[1];
    let mut adj = vec![Vec::new(); n as usize];
    for _ in 0..m{
        let input = read_line();
        let a = input[0];
        let b = input[1];
        adj[a as usize].push(b);
        adj[b as usize].push(a);
    }
    let path = read_line();
    let mut bread = Vec::new();
    for _ in 0..n{
        bread.push(false);
    }
    let added = added_bfs(path.clone(), adj.clone(), n);
    bread = pre_run(path.clone(), added.clone(), bread);
    bread = run(path, adj, bread);
    let mut count = 0;
    for b in bread{
        if b {
            count += 1;
        }
    } 
    println!("{}", count);
}

fn added_bfs(path: Vec<i64>, adj: Vec<Vec<i64>>, n: i64) -> Vec<i64>{
    let mut visited: HashSet<i64> = HashSet::new();
    let mut count = 0;
    let mut added = Vec::new();
    for _ in 0..n{
        added.push(0);
    }
    visited.insert(path[0]);
    added[path[0] as usize] = count;
    for j in path {
        count += 1;
        for i in &adj[j as usize]{
            if !visited.contains(i){
                visited.insert(*i);
                added[*i as usize] = count;
            }
        }
    }
    //println!("added = {:?}", added);
    added
}

fn pre_run(path: Vec<i64>, added: Vec<i64>, mut bread: Vec<bool>) -> Vec<bool> {
    let mut lowest = std::i64::MAX;
    for i in (0..path.len()).rev() {
        let node = path[i] as usize;
        if added[node] > lowest {
            bread[node] = true;
        }else if added[node] < lowest {
            lowest = added[node];
        }
    }
    //println!("bread = {:?}", bread);
    bread
}

fn run(path: Vec<i64>, adj: Vec<Vec<i64>>, mut bread: Vec<bool>) -> Vec<bool>{
    let mut changed;
    let mut breads;
    let mut queue;
    let mut visited;
    loop{
        changed = false;
        breads = 0;
        queue = VecDeque::new();
        visited = HashSet::new();
        queue.push_back(path[0]);
        visited.insert(path[0]);
        for i in &path{
            if breads > 0{ 
                breads -= 1;
                if !bread[*i as usize]{
                    changed = true;
                    bread[*i as usize] = true;
                }
            }
            for j in &adj[*i as usize]{
                if !visited.contains(j){
                    visited.insert(*j);
                    if bread[*j as usize]{
                        breads += 1;
                        queue.push_front(*i);
                    }else {
                        queue.push_back(*i);
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    //println!("bread = {:?}", bread);
    bread
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}

/* 
8 9
1 2
2 3
2 4
2 8
3 4
3 5 
3 7
4 5
5 6
1 2 3 5 4 6 7 8
*/