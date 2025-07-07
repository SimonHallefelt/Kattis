use std::{cmp::min, collections::HashSet, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let n = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().parse().unwrap();
    let mut locked_out = HashSet::with_capacity(n);
    let mut has_spare_key_to = vec![HashSet::new();n];
    let mut spare_key_to_me = vec![HashSet::new();n];
    for (i, line) in stdin.lock().lines().take(n).enumerate() {
        let row: Vec<usize> = line.unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        for j in 1..row[0]+1 {
            let has_spare = row[j]-1;
            has_spare_key_to[has_spare].insert(i);
            spare_key_to_me[i].insert(has_spare);
        }
    }

    let scc = scc(&has_spare_key_to);

    let mut node_has_left = Vec::with_capacity(n);
    let mut ids = vec![0;n];
    let mut id_has_spare_key_to = vec![HashSet::new();n];
    let mut spare_key_to_id = vec![HashSet::new();n];
    for (i, c) in scc.iter().enumerate() {
        node_has_left.push(c.len());
        for j in c {
            ids[*j] = i;
        }
    }
    for i in 0..n {
        let id = ids[i];
        for j in &spare_key_to_me[i] {
            let other_id = ids[*j];
            if id != other_id {
                spare_key_to_id[id].insert(other_id);
                id_has_spare_key_to[other_id].insert(id);
            }
        }
    }

    for line in stdin.lock().lines().take(n) {
        let left_house: usize = line.unwrap().trim().parse::<usize>().unwrap()-1;
        let id: usize = ids[left_house];
        node_has_left[id] -= 1;
        
        if node_has_left[id] == 0 {
            if spare_key_to_id[id].is_empty() {
                locked_out.extend(scc[id].clone());
                delete_edges(id, &mut spare_key_to_id, &id_has_spare_key_to, &scc, &ids, &node_has_left, &mut locked_out);
            }
        }

        print!("{} ", locked_out.len())
    }
}

fn delete_edges(id: usize, spare_key_to_id: &mut Vec<HashSet<usize>>, id_has_spare_key_to: &Vec<HashSet<usize>>, scc: &Vec<Vec<usize>>, ids: &Vec<usize>, node_has_left: &Vec<usize>, locked_out: &mut HashSet<usize>) {
    for other_id in &id_has_spare_key_to[id] {
        spare_key_to_id[*other_id].remove(&id);

        if node_has_left[*other_id] == 0 && !locked_out.contains(&scc[*other_id][0]) {
            if spare_key_to_id[*other_id].is_empty() {
                locked_out.extend(scc[*other_id].clone());
                delete_edges(*other_id, spare_key_to_id, id_has_spare_key_to, scc, ids, node_has_left, locked_out);
            }
        }
    }
}

fn scc(edges: &Vec<HashSet<usize>>) -> Vec<Vec<usize>> { // Tarjan's Strongly Connected Component
    let mut scc = Vec::with_capacity(edges.len());
    let mut stack = Vec::with_capacity(edges.len());
    let mut id_low = vec![(0, 0); edges.len()];
    let mut visited = HashSet::with_capacity(edges.len());
    let mut on_stack = HashSet::with_capacity(edges.len());


    for i in 0..edges.len() {
        if visited.insert(i) {
            dfs(i, edges, &mut scc, &mut stack, &mut id_low, &mut visited, &mut on_stack);
        }
    }

    scc
}

fn dfs(at: usize, edges: &Vec<HashSet<usize>>, scc: &mut Vec<Vec<usize>>, stack: &mut Vec<usize>, id_low: &mut Vec<(usize, usize)>, visited: &mut HashSet<usize>, on_stack: &mut HashSet<usize>) {
    stack.push(at);
    on_stack.insert(at);
    id_low[at] = (visited.len(), visited.len());

    for other in &edges[at] {
        if visited.insert(*other) {
            dfs(*other, edges, scc, stack, id_low, visited, on_stack);
        }
        if on_stack.contains(other) {
            id_low[at].1 = min(id_low[at].1, id_low[*other].1)
        }
    }

    if id_low[at].0 == id_low[at].1 {
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


// old: did not work
// fn main() {
//     let stdin = io::stdin();
//     let n = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().parse().unwrap();
//     let mut locked_out = HashSet::new();
//     let mut has_left = vec![false;n];
//     let mut has_spare_key_to = vec![HashSet::new();n];
//     let mut spare_key_to_me = vec![HashSet::new();n];
//     for i in 0..n {
//         let row: Vec<usize> = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
//         for j in 1..row[0]+1 {
//             let has_spare = row[j]-1;
//             has_spare_key_to[has_spare].insert(i);
//             spare_key_to_me[i].insert(has_spare);
//         }
//     }
//     dbg!(has_left.clone(), has_spare_key_to.clone(), spare_key_to_me.clone());

//     for _ in 0..n {
//         let left_house: usize = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().parse::<usize>().unwrap()-1;
//         has_left[left_house] = true;
//         look_for_spares_and_delete(left_house, &mut has_left, &mut has_spare_key_to, &mut spare_key_to_me, &mut locked_out);
        
//         print!("{} ", locked_out.len())
//     }
// }

// fn look_for_spares_and_delete(me: usize, has_left: &mut Vec<bool>, has_spare_key_to: &mut Vec<HashSet<usize>>, spare_key_to_me: &mut Vec<HashSet<usize>>, locked_out: &mut HashSet<usize>) {
//     let mut visited = HashSet::new();
//     let (_, _, pass) = look_for_spares(me, has_left, has_spare_key_to, spare_key_to_me, locked_out, &mut visited);

//     dbg!(pass, visited.clone());

//     if !pass {
//         locked_out.extend(visited.clone());
//         for no_spare in visited {
//             for i in has_spare_key_to[no_spare].clone() {
//                 remove_spare_key_to_me(i, no_spare, has_left, has_spare_key_to, spare_key_to_me, locked_out);
//             }
//         }
//     }
// }

// fn look_for_spares(me: usize, has_left: &mut Vec<bool>, has_spare_key_to: &mut Vec<HashSet<usize>>, spare_key_to_me: &mut Vec<HashSet<usize>>, locked_out: &mut HashSet<usize>, visited: &mut HashSet<usize>) -> (Vec<usize>, Vec<usize>, bool) {
//     if !has_left[me] {
//         return (vec![me], Vec::new(), true);
//     }

//     if locked_out.contains(&me) {
//         return (Vec::new(), vec![me], false);
//     }

//     if !visited.insert(me) {
//         return (Vec::new(), Vec::new(), false);
//     }

//     let mut add = Vec::new();
//     let remove = vec![me];
//     let mut pass = false;
//     let mut got_an_double_empty = false;

//     for other in spare_key_to_me[me].clone() {
//         let (a, r, p) = look_for_spares(other, has_left, has_spare_key_to, spare_key_to_me, locked_out, visited);
//         pass |= p;
//         if a.is_empty() && r.is_empty() {
//             got_an_double_empty = true;
//             continue;
//         } 
//         if !a.is_empty() {
//             spare_key_to_me[me].extend(a.clone());
//             add.extend(a);
//         }
//         if !r.is_empty() {
//             spare_key_to_me[me].remove(&r[0]);
//         }

//     }

//     if got_an_double_empty {
//         return (Vec::new(), Vec::new(), pass);
//     }

//     (add, remove, pass)
// }

// fn remove_spare_key_to_me(me: usize, remove: usize, has_left: &mut Vec<bool>, has_spare_key_to: &mut Vec<HashSet<usize>>, spare_key_to_me: &mut Vec<HashSet<usize>>, locked_out: &mut HashSet<usize>) {
//     spare_key_to_me[me].remove(&remove);
//     if has_left[me] && spare_key_to_me[me].is_empty() {
//         if locked_out.insert(me) {
//             for i in has_spare_key_to[me].clone() {
//                 remove_spare_key_to_me(i, me, has_left, has_spare_key_to, spare_key_to_me, locked_out);
//             }
//         }
//     } else if has_left[me] && !locked_out.contains(&me) {
//         look_for_spares_and_delete(me, has_left, has_spare_key_to, spare_key_to_me, locked_out);
//     }
// }

/*
4
2 2 4
1 3
1 1
1 1
1
2
3
4

0 0 0 4
*/

/*
6
2 2 4
1 3
1 1
2 1 5
1 6
1 4
1
2
3
4
5
6

0 0 0 0 0 6
*/

/*
6
1 2
1 1
2 2 4
1 5
1 3
1 5
4
1
3
5
6
2

0 0 0 0 0 6
*/