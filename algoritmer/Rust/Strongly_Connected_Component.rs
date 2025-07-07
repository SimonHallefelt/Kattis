use std::{cmp::min, collections::HashSet};

fn scc(edges: &Vec<Vec<usize>>) -> Vec<Vec<usize>> { // Tarjan's Strongly Connected Component
    // edges[i]: get all nodes that node i points to
    let mut scc = Vec::with_capacity(edges.len());
    let mut stack = Vec::with_capacity(edges.len());
    let mut id_and_low = vec![(0, 0); edges.len()];
    let mut visited = HashSet::with_capacity(edges.len());
    let mut on_stack = HashSet::with_capacity(edges.len());

    for i in 0..edges.len() {
        if visited.insert(i) {
            dfs(i, edges, &mut scc, &mut stack, &mut id_and_low, &mut visited, &mut on_stack);
        }
    }

    scc
}

fn dfs(at: usize, edges: &Vec<Vec<usize>>, scc: &mut Vec<Vec<usize>>, stack: &mut Vec<usize>, id_and_low: &mut Vec<(usize, usize)>, visited: &mut HashSet<usize>, on_stack: &mut HashSet<usize>) {
    stack.push(at);
    on_stack.insert(at);
    id_and_low[at] = (visited.len(), visited.len());

    for other in &edges[at] {
        if visited.insert(*other) {
            dfs(*other, edges, scc, stack, id_and_low, visited, on_stack);
        }
        if on_stack.contains(other) {
            id_and_low[at].1 = min(id_and_low[at].1, id_and_low[*other].1)
        }
    }

    if id_and_low[at].0 == id_and_low[at].1 {
        let mut v = Vec::new();
        while let Some(node) = stack.pop() {
            on_stack.remove(&node);
            v.push(node);
            if at == node {
                break;
            }
        }
        scc.push(v);
    }
}

fn main() {
    let edges = vec![vec![1], vec![2], vec![0]];
    let scc = scc(&edges);
    let answer = vec![vec![2,1,0]];
    assert_eq!(answer, scc);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scc_1() {
        let edges = vec![vec![1], vec![2], vec![0]];
        let scc = scc(&edges);
        let answer = vec![vec![2,1,0]];
        assert_eq!(answer, scc);
    }

    #[test]
    fn test_scc_2() {
        let edges = vec![vec![1], vec![2], vec![0,3], vec![4], vec![3]];
        let scc = scc(&edges);
        let answer = vec![vec![4,3], vec![2,1,0]];
        assert_eq!(answer, scc);
    }
    #[test]
    fn test_scc_3() {
        let edges = vec![vec![1], vec![2], vec![0], vec![4], vec![3]];
        let scc = scc(&edges);
        let answer = vec![vec![2,1,0], vec![4,3]];
        assert_eq!(answer, scc);
    }

}