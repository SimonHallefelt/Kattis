use std::collections::{VecDeque, HashSet};

fn bfs(start: i64, end: i64, nodes: Vec<Vec<i64>>) -> i64{
    let mut que: VecDeque<i64> = VecDeque::new();
    let mut visited: HashSet<i64> = HashSet::new();
    let mut node;
    let mut count = 0;
    que.push_back(start);
    visited.insert(start);
    while !que.is_empty() {
        node = que.pop_front().unwrap();
        visited.insert(node);
        if node == end {
            break;
        }
        for i in nodes[node as usize]{
            if !visited.contains(&i){
                que.push_back(i);
                visited.insert(i);
            }
        }
    }
    count
}

//tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bfs() {
    }
}