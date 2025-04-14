use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_5.txt"; // 5017
    let (befor, prints) = read_input(file_path);

    let mut tot = 0;
    let mut prints_wrong = Vec::new();
    'outer: for row in prints.clone() {
        for i in 0..row.len() {
            let b = befor.get(row.get(i).unwrap());
            if b.is_none() {
                continue;
            }
            for j in i+1..row.len() {
                if b.unwrap().contains(row.get(j).unwrap()) {
                    prints_wrong.push(row);
                    continue 'outer;
                }
            }
        }
    }


    for mut row in prints_wrong {
        let mut k = 0;
        'outer: loop {
            let mut correct = true;
            for mut i in k..row.len() {
                let b = befor.get(row.get(i).unwrap());
                if b.is_none() {
                    continue;
                }
                for j in i+1..row.len() {
                    if b.unwrap().contains(row.get(j).unwrap()) {
                        if correct {
                            k = i;
                            correct = false;
                        }
                        let v = *row.get(j).unwrap();
                        row.remove(j);
                        row.insert(i, v);
                        i += 1;
                    }
                }
                if !correct {
                    continue 'outer;
                }
            } 
            if correct {
                break;
            }
        }
        tot += row.get(row.len()/2).unwrap();
    }

    println!("{}", tot);
}

fn read_input(file: &str) -> (HashMap<i64, Vec<i64>>, Vec<Vec<i64>>) {
    let content = read_to_string(file).expect("file not found");
    let temp = content.trim().split("\n").collect::<Vec<&str>>();

    let mut befor: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut k = 0;
    for i in 0..temp.len() {
        let s = temp.get(i).unwrap().trim();
        if s.len() == 0 {
            k = i+1;
            break;
        }
        let ss = s.split("|").map(|a| a.trim().parse().unwrap()).collect::<Vec<i64>>();
        let f = *ss.first().unwrap();
        let l = *ss.last().unwrap();
        if befor.contains_key(&l) {
            befor.get_mut(&l).unwrap().push(f);
        } else {
            befor.insert(l, vec![f]);
        }
    }
    let mut prints = Vec::new();
    for i in k..temp.len() {
        let s = temp.get(i).unwrap();
        prints.push(s.split(",").map(|a| a.trim().parse().unwrap()).collect());
    }

    (befor, prints)
}