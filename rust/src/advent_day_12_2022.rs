use std::io;
use std::collections::HashMap;
use std::collections::LinkedList;

fn main() {
    let leters: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26];
    let mut map = HashMap::new();
    let mut matrix_map: Vec<Vec<char>> = Vec::new();
    let mut matrix_visited: Vec<Vec<bool>> = Vec::new();
    let mut queue: LinkedList<((i32, i32), i32)> = LinkedList::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);

    for i in 0..leters.len() {
        map.insert(leters[i], numbers[i]);
    }

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input: Vec<char> = input.trim().chars().collect();
        matrix_map.push(input);
        
        let mut visited_temp: Vec<bool> = Vec::new();
        for _ in 0..matrix_map[0].len() {
            visited_temp.push(false);
        }
        matrix_visited.push(visited_temp);

        let temp = matrix_map.len()-1;
        for j in 0..matrix_map[0].len() {
            if matrix_map[matrix_map.len()-1][j] == 'S' {
                start = (temp.to_string().parse::<i32>().unwrap() , j.to_string().parse::<i32>().unwrap());
                matrix_map[temp][j] = 'a';
            }
            if matrix_map[matrix_map.len()-1][j] == 'E' {
                end = (temp.to_string().parse::<i32>().unwrap(), j.to_string().parse::<i32>().unwrap());
                matrix_map[temp][j] = 'z';
            }
        }
    
        queue.clear();
        queue.push_back((start, 0));
        matrix_visited[start.0 as usize][start.1 as usize] = true;
        println!("start = {:?}", start);
        println!("end = {:?}", end);

        while !queue.is_empty() {
            let info = queue.pop_front().unwrap();
            let pos = info.0;
            let steps = info.1;
            if pos == end {
                println!("steps = {}", steps);
                break;
            }
            let hight: i32 = map.get(&matrix_map[pos.0 as usize][pos.1 as usize]).unwrap().to_string().parse::<i32>().unwrap() +2;
            if pos.0 > 0 && !matrix_visited[(pos.0-1) as usize][pos.1 as usize] {
                if map.get(&matrix_map[(pos.0-1) as usize][pos.1 as usize]).unwrap() < &hight {
                    queue.push_back(((pos.0-1, pos.1), steps+1));
                    matrix_visited[(pos.0-1) as usize][pos.1 as usize] = true;
                }
            }
            if (pos.0 as usize) < matrix_map.len()-1 && !matrix_visited[(pos.0+1) as usize][pos.1 as usize] {
                if map.get(&matrix_map[(pos.0+1) as usize][pos.1 as usize]).unwrap() < &hight {
                    queue.push_back(((pos.0+1, pos.1), steps+1));
                    matrix_visited[(pos.0+1) as usize][pos.1 as usize] = true;
                }
            }
            if pos.1 > 0 && !matrix_visited[pos.0 as usize][(pos.1-1) as usize] {
                if map.get(&matrix_map[pos.0 as usize][(pos.1-1) as usize]).unwrap() < &hight {
                    queue.push_back(((pos.0, pos.1-1), steps+1));
                    matrix_visited[pos.0 as usize][(pos.1-1) as usize] = true;
                }
            }
            if (pos.1 as usize) < matrix_map[pos.0 as usize].len()-1 && !matrix_visited[pos.0 as usize][(pos.1+1) as usize] {
                if map.get(&matrix_map[pos.0 as usize][(pos.1+1) as usize]).unwrap() < &hight {
                    queue.push_back(((pos.0, pos.1+1), steps+1));
                    matrix_visited[pos.0 as usize][(pos.1+1) as usize] = true;
                }
            }
        }

        for i in 0..matrix_visited.len() {
            for j in 0..matrix_visited[i].len() {
                matrix_visited[i][j] = false;
            }
        }
    }
}
