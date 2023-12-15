use std::{fs, collections::{HashMap, HashSet}};

fn main(){
    let mut total: i64 = 0;
    //let file_path = "src\\advent_of_code\\2023\\data\\day_8_test.txt";
    //let file_path = "src\\advent_of_code\\2023\\data\\day_8_test2.txt";
    //let file_path = "src\\advent_of_code\\2023\\data\\day_8_test3.txt";
    let file_path = "src\\advent_of_code\\2023\\data\\day_8.txt";
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
        movee(pos[i].clone(), end_points.clone(), map.clone(), lr.clone(), list[i].clone())
    }

    total = 1;
    loop {
        let mut sets = Vec::new();

        for i in 0..end_points.len() {
            let mut set = HashSet::new();
            for j in 0..list[i].2.len() {
                set.insert(list[i].0 + list[i].1 * (total-1) + list[i].2[j]);
            }
            sets.push(set);
        }


        let (intersection, others) = sets.split_at_mut(1);
        let intersection = &mut intersection[0];
        for other in others {
            intersection.retain(|e| other.contains(e));
        }


        if intersection.len() > 0 {
            total = *intersection.iter().min().unwrap();
            break;
        }
        total += 1;
    }

    /* let mut movee = 0;
    println!("start_points = {}", pos.len());
    while !found.iter().all(|x| !!x) {
        for i in 0..pos.len() {
            if lr[movee] == 'L' {pos[i] = map.get(&pos[i]).unwrap().0.clone()}
            else {pos[i] = map.get(&pos[i]).unwrap().1.clone()}
            if end_points.contains(&pos[i]) {found[i] = true}
            else {found[i] = false}
        }
        total += 1;
        movee += 1; movee %= lr.len();
        if total % 1000000 == 0 {println!("{}", total)}
    } */

    println!("total = {}", total);
}


fn movee(start: String, end_points: HashSet<String>, map: HashMap<String, (String, String)>, lr: Vec<char>, list: (i64, i64, Vec<i64>)) {
    let steps = list.0+list.2[0]+list.1;
    let mut movee = 0;
    let mut p = start;
    for i in 1..steps {
        if lr[movee] == 'L' {p = map.get(&p).unwrap().0.clone()}
        else {p = map.get(&p).unwrap().1.clone()}
        
        movee += 1; movee %= lr.len();
        if end_points.contains(&p) {
            println!("found end, i = {}, p = {}",i , p)
        }
    }
    println!("p = {}", p)
}


fn circular(start: String, end_points: HashSet<String>, map: HashMap<String, (String, String)>, lr: Vec<char>) -> (i64, i64, Vec<i64>) {
    let mut steps = 0;
    let mut circular = HashSet::new();
    let mut circular_info = HashMap::new();
    let circular_start;

    let mut movee = 0;
    let mut p = start;
    while circular.insert((p.clone(), movee)) {
        circular_info.insert(p.clone(), steps);

        if lr[movee] == 'L' {p = map.get(&p).unwrap().0.clone()}
        else {p = map.get(&p).unwrap().1.clone()}
        
        steps += 1;
        movee += 1; movee %= lr.len();
    }
    circular_start = p.clone();
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

    println!("{:?}",(*circular_info.get(&circular_start).unwrap() ,circular.len() as i64, list.clone()));
    (*circular_info.get(&circular_start).unwrap() ,circular.len() as i64, list)
}
