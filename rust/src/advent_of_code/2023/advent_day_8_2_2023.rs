use std::{fs, collections::{HashMap, HashSet}};

fn main(){
    let mut total: i128 = 0;
    //let file_path = "src\\advent_of_code\\2023\\data\\day_8_test.txt";
    //let file_path = "src\\advent_of_code\\2023\\data\\day_8_test2.txt";
    //let file_path = "src\\advent_of_code\\2023\\data\\day_8_test3.txt";
    let file_path = "D:\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_8.txt"; //13334102464297
    let contents = fs::read_to_string(file_path)            
        .expect("Should have been able to read the file");
    let mut lr: Vec<_> = Vec::new();
    let mut map = HashMap::new();
    let mut end_points = HashSet::new();

    let mut found = Vec::new();
    let mut pos = Vec::new();

    //println!("{}", contents);
    for c in contents.split("\n").enumerate() {
        if c.1.trim().len() == 0 {continue;}
        else if c.0 == 0 {lr = c.1.trim().chars().collect();}
        else {
            let s: Vec<&str> = c.1.trim().split(" ").collect();
            let k = s[0].to_string();
            let v = (s[2].trim().replace("(", "").replace(",", ""), 
                                        s[3].trim().replace(")", ""));
            map.insert(k.clone(), v);
            if k.clone().chars().nth(2).unwrap() == 'A' {
                found.push(false);
                pos.push(k);
            } else if k.clone().chars().nth(2).unwrap() == 'Z' {
                end_points.insert(k);
            }
        }
    }

    let mut list = Vec::new();
    for i in 0..end_points.len() {
        list.push(circular(pos[i].clone(), end_points.clone(), map.clone(), lr.clone()))
    }

    for i in 0..end_points.len() {
        check_moves(pos[i].clone(), end_points.clone(), map.clone(), lr.clone(), list[i].clone())
    }

    let mut steps = Vec::new();
    let start_cycles = 10000000000*2*2*2*2*2*2*2*2*2*2;
    for i in 0..pos.len() {
        let step = list[i].0+list[i].2[0]+(start_cycles/list[i].1) * list[i].1;
        steps.push(step)
    }
    println!("start_points = {}", steps.len());
    loop {
        let mut max_value = 0;
        for i in 0..steps.len() {
            if max_value < steps[i] {
                max_value = steps[i];
            }
        }
        for i in 0..steps.len() {
            if max_value > steps[i] {
                steps[i] += list[i].1;
            }
        }
        let mut b = true;
        for i in 1..steps.len() {
            if max_value != steps[i] {
                b = false;
                break;
            }
        }
        if b {break;}

        if max_value > total {
            println!("{}", total);
            total += 100000000000;
        }
        if start_cycles*2*list[0].1 + (list[0].1 * 100) < steps[0] {
            println!("fail at {}", total);
            return;
        }
    }

    println!("total = {}", steps[0]);
}


fn check_moves(start: String, end_points: HashSet<String>, map: HashMap<String, (String, String)>, lr: Vec<char>, list: (i128, i128, Vec<i128>)) {
    let steps = list.0+list.2[0]+list.1;
    let mut movee = 0;
    let mut p = start;
    for i in 0..steps {
        if lr[movee] == 'L' {p = map.get(&p).unwrap().0.clone()}
        else {p = map.get(&p).unwrap().1.clone()}
        
        movee += 1; movee %= lr.len();
        if end_points.contains(&p) {
            println!("found end, i = {}, p = {}",i , p)
        }
    }
    println!("p = {}", p)
}


fn circular(start: String, end_points: HashSet<String>, map: HashMap<String, (String, String)>, lr: Vec<char>) -> (i128, i128, Vec<i128>) {
    let mut steps = 0;
    let mut circular = HashSet::new();

    let mut movee = 0;
    let mut p = start;
    while circular.insert((p.clone(), movee)) {
        if lr[movee] == 'L' {p = map.get(&p).unwrap().0.clone()}
        else {p = map.get(&p).unwrap().1.clone()}
        
        steps += 1;
        movee += 1; movee %= lr.len();
    }
    let circular_steps = steps;
    steps = 0;
    circular = HashSet::new();
    let mut list = Vec::new();
    while circular.insert((p.clone(), movee)) {
        if lr[movee] == 'L' {p = map.get(&p).unwrap().0.clone()}
        else {p = map.get(&p).unwrap().1.clone()}
        
        steps += 1;
        movee += 1; movee %= lr.len();

        if end_points.contains(&p) {
            list.push(steps);
        }
    }

    println!("{:?}",(circular_steps, circular.len() as i128, list.clone()));
    (circular_steps ,circular.len() as i128, list)
}
