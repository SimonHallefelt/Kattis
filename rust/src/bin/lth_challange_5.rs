use std::{collections::{HashSet, VecDeque}, io};

fn main() {
    let input = read_line();
    let input: Vec<&str> = input.split(" ").collect();
    let r: i32 = input[0].parse().unwrap();
    let c: i32 = input[1].parse().unwrap();
    let n: i32 = input[2].parse().unwrap();

    let mut grid = Vec::new();
    let mut robots = Vec::new();
    let mut count = 0;
    for i in 0..r {
        let mut v = Vec::new();
        let input = read_line();
        for ch in input.chars() {
            if ch == '.' {
                v.push(0);
            } else if ch == '#' {
                v.push(1);
            } else if ch == 'P' {
                v.push(2 + count);
                count += 1;
            } else {
                v.push(0);
                robots.push((i, v.len() as i32 -1))
            } 
        }
        grid.push(v);
    }
    // println!("{:?}", grid);

    let mut v = Vec::new();
    for rob in robots {
        let temp = bfs(rob, &grid, (r,c));
        v.push(temp)
    }

    let mut vv = Vec::new();
    let mut rr = Vec::new();
    for _ in 0..n {
        vv.push(Vec::new());
        rr.push(Vec::new());
    }

    for i in 0..n {
        for ttv in &v[i as usize] {
            rr[i as usize].push(ttv.0);
        }
    }

    for tv in v {
        for ttv in tv {
            vv[(ttv.1-2) as usize].push(ttv.0)
        }
    }

    let mut vvv = Vec::new();
    for tv in vv {
        let t = *tv.iter().min().unwrap();
        vvv.push(t);
    }

    for tv in rr {
        let t = *tv.iter().min().unwrap();
        vvv.push(t);
    }

    let t = vvv.iter().max().unwrap();
    println!("{}", t);
}

fn bfs(start: (i32, i32), grid: &Vec<Vec<i32>>, dim: (i32, i32)) -> Vec<(i32, i32)>{
    let mut que = VecDeque::new();
    let mut bla = VecDeque::new();
    let mut visited = HashSet::new();
    let mut node;
    let mut count = 1;
    que.push_back(start);
    bla.push_back(count);
    visited.insert(start);
    let dir = vec![(0,1),(1,0),(-1,0),(0,-1)];
    let mut p = Vec::new();
    while !que.is_empty() {
        node = que.pop_front().unwrap();
        count = bla.pop_front().unwrap();
        visited.insert(node);
        for d in &dir {
            let r = node.0 + d.0;
            let c = node.1 + d.1;
            if r < 0 || c < 0 {
                continue;
            }
            if r >= dim.0 || c >= dim.1 {
                continue;
            }
            let t = grid[r as usize][c as usize];
            if t >= 2 {
                p.push((count, t));
            } else if t == 1 {
                continue;
            }
            let i = (r,c);
            if !visited.contains(&i){
                que.push_back(i);
                bla.push_back(count+1);
                visited.insert(i);
            }
        }
    }
    p
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}