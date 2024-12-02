use std::{fs, collections::{HashMap, VecDeque, HashSet, BinaryHeap}, cmp::Ordering};

#[derive(Clone, Eq, PartialEq, Debug)]
struct Node {
    pos:(usize, usize),
    go_to:Vec<(usize, (usize, usize))>,
}


#[derive(Clone, Eq, PartialEq, Debug)]
struct Walker {
    pos:(usize, usize),
    steps:usize,
    visited:HashSet<(usize, usize)>,
}


fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_23_test.txt"; // 94
    let file_path = "src\\advent_of_code\\2023\\data\\day_23.txt"; // 2018
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = Vec::new();
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    for c in contents.trim().split("\n") {
        let line = fix_input(c);
        if map.is_empty() {start = find_start(line.clone())}
        map.push(line);
    }
    end = find_end(map[map.len()-1].clone(), map.len()-1);

    println!("enter build_graph");
    let graph = build_graph(&mut map, start, end);
    println!("graph.len() = {}", graph.len());
    total += run(&graph, start, end);

    println!("total = {}", total);
}


fn run(graph: &HashMap<(usize,usize), Node>, start: (usize,usize), end: (usize,usize)) -> i64 {
    let mut max_steps = 0;
    let mut walking = Vec::new();
    walking.push(Walker { pos: start, steps: 0, visited: HashSet::new() });

    while !walking.is_empty() {
        let w = walking.pop().unwrap();
        for n in graph.get(&w.pos).unwrap().go_to.clone() {
            let mut new_w = w.clone();
            if new_w.visited.insert(n.1) {
                new_w.pos = n.1;
                new_w.steps += n.0;
                if n.1 == end {
                    if new_w.steps > max_steps {
                        max_steps = new_w.steps;
                    }
                } else {
                    walking.push(new_w)
                }
            }
        }
    }

    max_steps as i64
}


fn build_graph(map: &Vec<Vec<char>>, start: (usize,usize), end: (usize,usize)) -> HashMap<(usize,usize), Node> {
    let mut graph = HashMap::new();
    graph.insert(start, Node { pos: start, go_to: Vec::new() });
    graph.insert(end, Node { pos: end, go_to: Vec::new() });

    let mut nodes_left = Vec::new();
    nodes_left.push(start);
    while !nodes_left.is_empty() {
        //println!("hej, build_graph");
        let node = nodes_left.pop().unwrap();
        let can_go_to = find_nodes(map, node, end);
        //println!("can_go_to.len() = {}", can_go_to.len());
        graph.get_mut(&node).unwrap().go_to = can_go_to.clone();
        for n in can_go_to {
            if n.1 == end {continue;}
            if !graph.contains_key(&n.1) {
                nodes_left.push(n.1);
                graph.insert(n.1, Node { pos: n.1, go_to: Vec::new() });
            }
        }
    }

    graph
}


fn find_nodes(map: &Vec<Vec<char>>, start: (usize,usize), end: (usize,usize)) -> Vec<(usize, (usize, usize))> {
    println!("enter find_nodes");
    let mut found = Vec::new();
    let directions = Vec::from([(0,1), (-1,0), (0,-1), (1,0)]);
    let directions_symbol: HashMap<char, (i64, i64)> = HashMap::from([('<', (0,1),), ('v', (-1,0)), ('>', (0,-1)), ('^', (1,0))]);
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut walk = Vec::new();
    for d in directions.clone() {
        let new_pos = (start.0 as i64+d.0, start.1 as i64+d.1);
        if new_pos.0 < 0 {continue;}
        let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
        if map[new_pos.0][new_pos.1] == '#' {continue;}

        if map[new_pos.0][new_pos.1] == '.' {
            walk.push((1, new_pos));
            visited.insert(new_pos);
        }else if *directions_symbol.get(&map[new_pos.0][new_pos.1]).unwrap() != d {
            println!("d = {:?}, new_pos = {:?}", 
                d, directions_symbol.get(&map[new_pos.0][new_pos.1]).unwrap());
            walk.push((1, new_pos));
            visited.insert(new_pos);
        }
    }
    //println!("hej, build_graph, walk = {:?}", walk);

    let directions_symbol: HashMap<char, (i64, i64)> = HashMap::from([('>', (0,1),), ('^', (-1,0)), ('<', (0,-1)), ('v', (1,0))]);
    while !walk.is_empty() {
        let mut pos = walk.pop().unwrap();
        if map[pos.1.0][pos.1.1] != '.' {
            let temp = directions_symbol.get(&map[pos.1.0][pos.1.1]).unwrap();
            pos.1 = ((pos.1.0 as i64+temp.0) as usize, (pos.1.1 as i64+temp.1) as usize);
            pos.0 += 1; // ska denna finnas
            visited.insert(pos.1);
        }

        let mut n = Vec::new();
        for d in directions.clone() {
            let new_pos = (pos.1.0 as i64+d.0, pos.1.1 as i64+d.1);
            if new_pos.0 < 0 {continue;}
            let new_pos = (new_pos.0 as usize, new_pos.1 as usize);
            if map[new_pos.0][new_pos.1] == '#' {continue;}
            if new_pos == start {continue;}
            n.push(new_pos);
        }

        //println!("hej, build_graph, n = {:?}", n);
        
        if n.len() > 2 {
            found.push(pos)
        } else {
            for p in n {
                if p == end {
                    found.push((pos.0+1, p))
                } else if visited.insert(p) {
                    walk.push((pos.0+1, p))
                }
            }
        }
    }

    //clean found
    //println!("found.len() = {}", found.len());
    found.sort_by(|a, b| a.0.cmp(&b.0));
    let mut temp = HashSet::new();
    let mut new_found = Vec::new();
    for f in found {
        if temp.insert(f.1) {
            new_found.push(f);
        }
    }
    
    new_found 
}


fn find_end(line: Vec<char>, row: usize) -> (usize, usize) {
    let mut end = (row, usize::MAX);

    for i in 0..line.len() {
        if line[i] == '.' {
            end.1 = i;
            break;
        }
    }

    end
}


fn find_start(line: Vec<char>) -> (usize, usize) {
    let mut start = (0, usize::MAX);

    for i in 0..line.len() {
        if line[i] == '.' {
            start.1 = i;
            break;
        }
    }

    start
}


fn fix_input(line: &str) -> Vec<char> {
    line.trim().chars().collect()
}
