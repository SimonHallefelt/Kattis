use std::{cmp::{min, Reverse}, collections::{BinaryHeap, HashSet}, io::{self, BufRead}, usize::{self}};

fn main() {
    let stdin = io::stdin();
    let row: Vec<usize> = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (a, f) = (row[0], row[1]);

    let mut nodes = Vec::with_capacity(a);
    for i in 0..a {
        nodes.push(Node::new(i));
    }
    let mut a380 = Vec::new();

    for line in stdin.lock().lines().take(f) {
        let row = line.expect("could not read line");
        let data: Vec<&str> = row.trim().split_whitespace().collect();
        let (o, d, c, m) = (data[0].parse::<usize>().unwrap(), data[1].parse().unwrap(), data[2].parse().unwrap(), data[3]);

        nodes[o].push_can_reach(d, c);
        nodes[d].push_reach_me(o, c);

        if m == "A380" {
            a380.push((o, d, c));
        }
    }

    dijkstra(&mut nodes, true, (0, 0));
    dijkstra(&mut nodes, false, (0, a - 1));

    let mut cheapest = i64::MAX;
    for (o,d,c) in a380 {
        let dist_s = nodes[o].dist_s;
        let dist_e = nodes[d].dist_e;
        if dist_s == -1 || dist_e == -1 {continue;}
        cheapest = min(cheapest, dist_s + dist_e + c as i64);
    }
    
    if cheapest == i64::MAX {
        println!("-1")
    } else {
        println!("{}", cheapest)
    }
}

fn dijkstra(nodes: &mut Vec<Node>, forward: bool, start: (i64, usize)) {
    let mut pq = BinaryHeap::new();
    pq.push(Reverse(start));
    let mut visited = HashSet::new();
    while let Some(Reverse((dist, pos))) = pq.pop() {
        if visited.insert(pos) {
            nodes[pos].set_dist(forward, dist);
            for neighbor in nodes[pos].neighbors(forward) {
                if visited.contains(&neighbor.0) {continue;}
                pq.push(Reverse((dist + neighbor.1 as i64, neighbor.0)));
            }
        }
    }
}


impl Node {
    fn new(name: usize) -> Self {
        let can_reach = Vec::new();
        let reach_me = Vec::new();
        let dist_s = -1;
        let dist_e = -1;
        Self { name, can_reach, reach_me, dist_s, dist_e }
    }

    fn push_can_reach(&mut self, d: usize, c: usize) {
        self.can_reach.push((d,c));
    }

    fn push_reach_me(&mut self, o: usize, c: usize) {
        self.reach_me.push((o,c));
    }

    fn set_dist(&mut self, forward: bool, dist: i64) {
        if forward {
            self.dist_s = dist;
        } else {
            self.dist_e = dist;
        }
    }

    fn neighbors(&self, forward: bool) -> &Vec<(usize,usize)> {
        match forward {
            true => &self.can_reach,
            false => &self.reach_me
        }
    }
}

struct Node {
    name: usize,
    can_reach: Vec<(usize,usize)>,
    reach_me: Vec<(usize,usize)>,
    dist_s: i64,
    dist_e: i64,
}