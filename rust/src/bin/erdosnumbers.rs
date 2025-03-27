use std::{collections::{HashMap, HashSet, VecDeque}, io::{self, BufRead}};

fn main() {
    let input = read_input();

    let mut start_nodes = Vec::new();
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input {
        start_nodes.push(line.get(0).unwrap().clone());
        if !graph.contains_key(line.get(0).unwrap()) {
            graph.insert(line.get(0).unwrap().clone(), HashSet::new());
        }
        for (i, name) in line.iter().enumerate() {
            if i == 0 {continue;}
            let hs = graph.get_mut(line.get(0).unwrap()).unwrap();
            hs.insert(name.clone());
            if let Some(hs2) = graph.get_mut(name) {
                hs2.insert(start_nodes.last().unwrap().clone());
            } else {
                graph.insert(name.clone(), HashSet::from([start_nodes.last().unwrap().clone()]));
            }
        }
    }

    let end_node = "PAUL_ERDOS".to_string();
    let (_, hm) = bfs(&graph, end_node.clone(), "bla".to_string());
    for st in start_nodes {
        if st == end_node {
            println!("{} 0", st);
        } else if let Some(n) = hm.get(&st) {
            println!("{} {}", st, *n)
        } else {
            println!("{} no-connection", st)
        }
    }

}

fn bfs2(graph: &HashMap<i32, HashSet<i32>>, start_node: i32, end_node: i32) -> Vec<i32> {
    let mut previous_node = HashMap::new();
    let mut queue = VecDeque::from(vec![start_node]);
    let mut path = Vec::new();
    if start_node == end_node {
        return vec![end_node];
    }

    'outer: while let Some(current_node) = queue.pop_front() {
        for mut neighbor in graph.get(&current_node).unwrap() {
            if !previous_node.contains_key(neighbor) {
                previous_node.insert(neighbor, current_node);
                queue.push_back(*neighbor);
                if *neighbor == end_node {
                    path.push(*neighbor);
                    while *neighbor != start_node {
                        neighbor = previous_node.get(neighbor).unwrap();
                        path.push(*neighbor);
                    }
                    break 'outer;
                }
            }
        }
    }

    path.reverse();
    path
}

fn bfs(graph: &HashMap<String, HashSet<String>>, start_node: String, end_node: String) -> (Vec<String>, HashMap<String, i32>){
    let mut previous_node = HashMap::from([(start_node.clone(),start_node.clone())]);
    let mut queue = VecDeque::from(vec![start_node.clone()]);
    let mut path = Vec::new();
    let mut distances = HashMap::from([(start_node.clone(), 0)]);
    if start_node == end_node {
        return (vec![end_node.to_string()], distances);
    }

    'outer: while let Some(current_node) = queue.pop_front() {
        for mut neighbor in graph.get(&current_node).unwrap() {
            if !previous_node.contains_key(neighbor) {
                previous_node.insert(neighbor.clone(), current_node.clone());
                queue.push_back(neighbor.clone());
                distances.insert(neighbor.clone(), distances.get(&current_node).unwrap()+1);
                if *neighbor == end_node {
                    path.push(neighbor.to_string());
                    while *neighbor != start_node {
                        neighbor = previous_node.get(neighbor).unwrap();
                        path.push(neighbor.to_string());
                    }
                    break 'outer;
                }
            }
        }
    }

    path.reverse();
    (path, distances)
}

fn read_input() -> Vec<Vec<String>> {
    let mut lines = vec![];
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                if input.trim().is_empty() {break;}
                lines.push(input.trim().split(" ").map(|s| s.to_string()).collect())
            }
            Err(_) => {
                break;
            }
        }
    }

    lines
}