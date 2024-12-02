use std::{fs, collections::{HashMap, VecDeque, HashSet}};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_20_test.txt"; //32000000
    //let file_path = "src\\advent_of_code\\2023\\data\\day_20_test2.txt"; //11687500
    //let file_path = "src\\advent_of_code\\2023\\data\\day_20_test3.txt"; //
    let file_path = "src\\advent_of_code\\2023\\data\\day_20.txt"; //812609846
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = HashMap::new();
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        map.insert(input.0, input.1);
    }

    find_pointers(&mut map);
    total += run(map);

    println!("total = {}", total);
}


fn run(mapp: HashMap<String, (bool, Vec<String>, Vec<(String, bool)>)>) -> i64 {
    let mut positiv = 0;
    let mut negative = 0;
    let mut queue = VecDeque::new();
    let mut hs = HashSet::new();

    let mut i = 0;
    let mut map = mapp.clone();
    while i < 1000 {
        let m = map.clone();
        if hs.insert(Vec::from_iter(m)){

        }
        queue.push_back(("broadcaster".to_string(), false, "".to_string()));
        while !queue.is_empty() {
            let q = queue.pop_front().unwrap();
            if q.1 {
                positiv += 1;
            }else {
                negative += 1;
            }
            let mut out_signal = false;
            //println!("q = {:?},  {:?}", q, map.get(&q.0));
            if !map.contains_key(&q.0) {continue;}
            if q.0 == "broadcaster" {
                out_signal = false;
            } else if map.get(&q.0).unwrap().0 {
                for k in 0..map.get(&q.0).unwrap().2.len() {
                    let temp = map.get(&q.0).unwrap().2[k].clone();
                    if temp.0 == q.2 {
                        map.get_mut(&q.0).unwrap().2[k].1 = q.1;
                        break;
                    }
                }
                out_signal = false;
                for k in 0..map.get(&q.0).unwrap().2.len() {
                    if !map.get(&q.0).unwrap().2[k].1 {
                        out_signal = true;
                        break;
                    }
                }
            } else {
                let temp = map.get(&q.0).unwrap().2[0].1;
                if q.1 {
                    //out_signal = temp;
                    continue;
                } else {
                    out_signal = !temp;
                    map.get_mut(&q.0).unwrap().2[0].1 = !temp;
                }
            }
            for s in map.get(&q.0).unwrap().1.clone() {
                queue.push_back((s, out_signal, q.0.clone()));
            }
        }

        i += 1;
    }

    println!("positiv = {}, negative = {}", positiv, negative);
    positiv * negative
}


fn find_pointers(map: &mut HashMap<String, (bool, Vec<String>, Vec<(String, bool)>)>) {
    let mut points: HashMap<String, Vec<(String, bool)>> = HashMap::new();
    let mut to_do = Vec::new();
    for m in map.clone() {
        if m.1.0 {
            to_do.push(m.0.clone());
        }
        for p in m.1.1.clone() {
            let temp = (m.0.clone(), false);
            if points.contains_key(&p) {
                points.get_mut(&p).unwrap().push(temp);
            }  else {
                points.insert(p, Vec::from([temp]));
            }
        }
    }

    for p in to_do {
        map.get_mut(&p).unwrap().2 = points.get(&p).unwrap().clone();
        /* for _ in 0..*points.get(&p).unwrap() {
            map.get_mut(&p).unwrap().2.push(false);
        } */
    }
}


fn fix_input(line: &str) -> (String, (bool, Vec<String>, Vec<(String, bool)>)) {
    let temp: Vec<String> = line.trim().split(" ").map(|x| x.to_string()).collect();
    let name;
    let mut t = false;
    if temp[0].trim() == "broadcaster" {
        name = temp[0].clone();
    } else if '%' == temp[0].trim().chars().nth(0).unwrap() {
        name = temp[0].trim().replace("%", "");
        t = false;
    }else {
        name = temp[0].trim().replace("&", "");
        t = true;
    }
    let mut to = Vec::new();
    for i in 2..temp.len() {
        to.push(temp[i].trim().replace(",", ""))
    }
    (name, (t, to, Vec::from([("".to_string(), false)])))
}
