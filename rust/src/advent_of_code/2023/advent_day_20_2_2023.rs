use std::{fs, collections::{HashMap, VecDeque, HashSet}};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_20_test.txt"; //
    //let file_path = "src\\advent_of_code\\2023\\data\\day_20_test2.txt"; //
    //let file_path = "src\\advent_of_code\\2023\\data\\day_20_test3.txt"; //
    let file_path = "src\\advent_of_code\\2023\\data\\day_20.txt"; //
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


fn run(mapp: HashMap<String, (bool, Vec<String>, HashMap<String, bool>)>) -> i64 {
    let mut queue = VecDeque::new();
    /* let mut positiv = 0;
    let mut negative = 0; */
    //let mut hs = HashSet::new();

    let mut i = 0;
    let mut map = mapp.clone();
    while i < 100000000 {
        i += 1;
        /* let m = map.clone();
        if !hs.insert(Vec::from_iter(m)){
            println!("---FEL---");
            return 0;
        } */
        //negative += 1;
        for s in &map.get(&"broadcaster".to_string()).unwrap().1 {
            queue.push_back((s.clone(), false, "broadcaster".to_string()));
        }
        while !queue.is_empty() {
            let q = queue.pop_front().unwrap();
            if q.0 == "rx" && !q.1 {
                println!{"did it"}
                return i;
            }
            /* if q.1 {
                positiv += 1;
            }else {
                negative += 1;
            } */
            let mut out_signal = false;
            //println!("q = {:?},  {:?}", q, map.get(&q.0));
            if !map.contains_key(&q.0) {continue;}
            if map.get(&q.0).unwrap().0 {
                map.get_mut(&q.0).unwrap().2.insert(q.2, q.1);
                for k in map.get(&q.0).unwrap().2.values() {
                    if !k {
                        out_signal = true;
                        break;
                    }
                }
            } else {
                if q.1 {
                    continue;
                } else {
                    let temp = map.get(&q.0).unwrap().2.get("").unwrap();
                    out_signal = !temp;
                    map.get_mut(&q.0).unwrap().2.insert("".to_string(), out_signal);
                }
            }
            for s in &map.get(&q.0).unwrap().1 {
                queue.push_back((s.clone(), out_signal, q.0.clone()));
            }
        }
        if i % 100000 == 0{
            println!("i = {}", i);
            println!("hp = {:?}", map.get(&"hp".to_string()).unwrap().2);
        };
    }

    //positiv*negative
    0
}


fn find_pointers(map: &mut HashMap<String, (bool, Vec<String>, HashMap<String, bool>)>) {
    let mut points: HashMap<String, HashMap<String, bool>> = HashMap::new();
    let mut to_do = Vec::new();
    for m in map.clone() {
        if m.1.0 {
            to_do.push(m.0.clone());
        }
        for p in m.1.1.clone() {
            let temp = (m.0.clone(), false);
            if points.contains_key(&p) {
                points.get_mut(&p).unwrap().insert(temp.0, temp.1);
            }  else {
                points.insert(p, HashMap::from([temp]));
            }
        }
    }

    for p in to_do {
        map.get_mut(&p).unwrap().2 = points.get(&p).unwrap().clone();
    }
}


fn fix_input(line: &str) -> (String, (bool, Vec<String>, HashMap<String, bool>)) {
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
    (name, (t, to, HashMap::from([("".to_string(), false)])))
}
