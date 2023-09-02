use std::{io, collections::{HashMap, VecDeque}};

fn main(){
    let mut input = read_line();
    let num = input[0].parse::<i64>().unwrap();
    let mut map: HashMap<String, bool> = HashMap::new();
    input = read_line();
    for i in 0..num{
        let mut a = false;
        if input[i as usize] == "T" {a = true}
        map.insert(((i+65) as u8 as char).to_string(), a);
    }
    input = read_line();
    let mut deque = VecDeque::new();
    for i in input{
        if i == "*" {
            let a = deque.pop_back().unwrap();
            let b = deque.pop_back().unwrap();
            if a && b {
                deque.push_back(true);
            }else {
                deque.push_back(false);
            }
            //println!("{}, {}", i, a && b);
        }else if i == "+" {
            let a = deque.pop_back().unwrap();
            let b = deque.pop_back().unwrap();
            if a || b {
                deque.push_back(true);
            }else {
                deque.push_back(false);
            }
            //println!("{}, {}", i, a || b);
        }else if i == "-" {
            let a = deque.pop_back().unwrap();
            if !a {
                deque.push_back(true);
            }else {
                deque.push_back(false);
            }
            //println!("{}, {}", i, !a);
        }else {
            let a = *map.get(&i).unwrap();
            if a {
                deque.push_back(true);
            }else {
                deque.push_back(false);
            }
        }
        //println!("{}", i);
    }
    let a = deque.pop_back().unwrap();
    if a {
        println!("T");
    }else {
        println!("F");
    }
    
}

fn read_line() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().split(" ").map(|x| x.to_string()).collect()
}