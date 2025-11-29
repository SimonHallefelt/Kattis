use std::{collections::{HashSet, VecDeque}, io};

fn main() {
    let mut input = read_line_stdin();
    let (n,m,k) = (input[0], input[1], input[2]);
    let mut graph = vec![HashSet::new(); n];
    for _ in 0..m {
        input = read_line_stdin();
        let (a, b) = (input[0]-1, input[1]-1);
        graph[a].insert(b);
        graph[b].insert(a);
    }

    let mut visited = HashSet::with_capacity(n);
    let mut component = vec![0; n];
    for i in 0..n {
        if visited.contains(&i) {
            continue;
        }
        let hs = bfs(&graph, i);
        if hs.len() < k {
            println!("impossible");
            return;
        }
        hs.iter().for_each(|v| { visited.insert(*v); });
        for (i, v) in hs.iter().enumerate() {
            component[*v] = (i%k)+1;
        }
    }

    println!("{}", component.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "))
}

fn bfs(graph: &Vec<HashSet<usize>>, start_node: usize) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::from(vec![start_node]);

    while let Some(current_node) = queue.pop_front() {
        for neighbor in &graph[current_node] {
            if visited.insert(*neighbor) {
                queue.push_back(*neighbor);
            }
        }
    }

    visited
}

fn read_line_stdin() -> Vec<usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();
    input.split_whitespace().map(|f| f.parse().unwrap()).collect()
}