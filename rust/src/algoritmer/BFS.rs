use std::collections::{HashMap, HashSet, VecDeque};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let graph = HashMap::from([
            (1, HashSet::from([2,3])),
            (2, HashSet::from([3,4])),
            (3, HashSet::from([2,3,4,5])),
            (4, HashSet::from([2,3,4])),
            (5, HashSet::from([4,6])),
            (6, HashSet::from([5])),
            (7, HashSet::from([8])),
            (8, HashSet::from([7]))
        ]);
        assert_eq!(1, bfs(&graph, 1, 1).len());
        assert_eq!(4, bfs(&graph, 1, 6).len());
        assert_eq!(0, bfs(&graph, 1, 8).len());
        assert_eq!(2, bfs(&graph, 7, 8).len());
        assert_eq!(2, bfs(&graph, 8, 7).len());

        assert_ne!(0, bfs(&graph, 7, 7).len());
    }
}