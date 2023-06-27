use std::{io, collections::HashMap};



fn main(){
    let input = read_line();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let r = input[0].to_string().parse::<i128>().unwrap();
    let c = input[1].to_string().parse::<i128>().unwrap();

    let mut map : Vec<Vec<char>> = Vec::new();
    for _ in 0..r {
        let input = read_line();
        let input: Vec<char> = input.trim().chars().collect();
        map.push(input);
    }

    let input = read_line();
    let n = input.to_string().parse::<i128>().unwrap();
    let mut querys: Vec<Vec<i128>> = Vec::new();
    for _ in 0..n {
        let input = read_line();
        let input: Vec<&str> = input.trim().split(" ").collect();
        let mut vec: Vec<i128> = Vec::new();
        for i in 0..4 {
            vec.push(input[i as usize].to_string().parse::<i128>().unwrap());
        }
        querys.push(vec);
    }

    
    run(r, c, map, n, querys);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}



fn run(r: i128, c: i128, map: Vec<Vec<char>>, n: i128, querys: Vec<Vec<i128>>){
    let mut conencted: HashMap<(i128, i128), i128> = HashMap::new();
    let mut query: Vec<i128>;
    for i in 0..n {
        query = querys[i as usize].clone();
        let pos1 = (query[0]-1, query[1]-1);
        let pos2 = (query[2]-1, query[3]-1);

        //already exists and is connected
        if !conencted.contains_key(&pos1) && !conencted.contains_key(&pos2){
            if map[pos1.0 as usize][pos1.1 as usize] == 
                    map[pos2.0 as usize][pos2.1 as usize]{ //none of them existed alredy, but same type
                bfs(r, c, &map, pos1, &mut conencted, map[pos1.0 as usize][pos1.1 as usize], i);
            }
        }
        if conencted.contains_key(&pos1) && conencted.contains_key(&pos2){
            if conencted.get(&pos1) == conencted.get(&pos2){
                println!("{}", if map[pos1.0 as usize][pos1.1 as usize] == '0'{"binary"} else {"decimal"});
            } else {
                println!("neither");
            }
        }else {
            println!("neither");
        }
    }
}

fn bfs(r: i128, c: i128, map: &Vec<Vec<char>>, pos: (i128, i128), 
            conencted: &mut HashMap<(i128, i128), i128>, typ: char, i: i128){
    if pos.0-1 >= 0{ //up
        let new_pos = (pos.0-1, pos.1);
        if !conencted.contains_key(&new_pos){
            if map[new_pos.0 as usize][new_pos.1 as usize] == typ{
                conencted.insert(new_pos, i);
                bfs(r, c, &map, new_pos, conencted, typ, i)
            }
        }
    }

    if pos.0+1 < r{ //down
        let new_pos = (pos.0+1, pos.1);
        if !conencted.contains_key(&new_pos){
            if map[new_pos.0 as usize][new_pos.1 as usize] == typ{
                conencted.insert(new_pos, i);
                bfs(r, c, &map, new_pos, conencted, typ, i)
            }
        }
    }

    if pos.1-1 >= 0{ //left
        let new_pos = (pos.0, pos.1-1);
        if !conencted.contains_key(&new_pos){
            if map[new_pos.0 as usize][new_pos.1 as usize] == typ{
                conencted.insert(new_pos, i);
                bfs(r, c, &map, new_pos, conencted, typ, i)
            }
        }
    }

    if pos.1+1 < c{ //right
        let new_pos = (pos.0, pos.1+1);
        if !conencted.contains_key(&new_pos){
            if map[new_pos.0 as usize][new_pos.1 as usize] == typ{
                conencted.insert(new_pos, i);
                bfs(r, c, &map, new_pos, conencted, typ, i)
            }
        }
    }
}