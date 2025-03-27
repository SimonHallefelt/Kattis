fn bfs(graph: &HashMap<i32, HashSet<i32>>, start_node: i32, end_node: i32) -> Vec<i32> {
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