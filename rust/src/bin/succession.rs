use std::{io, collections::{HashMap, HashSet}};

fn main(){
    let mut v = Vec::new();
    let input = read_line();
    let n = input[0].parse::<i128>().unwrap();
    let m = input[1].parse::<i128>().unwrap();
    let mut hm: HashMap<String, (i128, i128)> = HashMap::new();
    let mut have_parents: HashMap<String, (String,String)> = HashMap::new();
    let mut all: HashSet<String> = HashSet::new();

    let input = read_line();
    hm.insert(input[0].clone(),(1 as i128, 1 as i128));
    for _ in 0..n {
        let input = read_line();
        have_parents.insert(input[0].clone(), (input[1].clone(),input[2].clone()));
        all.insert(input[1].clone());
        all.insert(input[2].clone());
    }

    for _ in 0..m {
        let input = read_line();
        v.push(input[0].clone());
    }

    for p in all {
        if !have_parents.contains_key(&p) {
            hm.insert(p, (0,1));
        }
    }

    let mut change = true;
    while change {
        change = false;
        for hp in have_parents.keys() {
            let p1 = have_parents.get(hp).unwrap().clone();
            if hm.contains_key(&p1.0) && hm.contains_key(&p1.1){
                change = true;
                let top = hm.get(&p1.0).unwrap().clone().0 * hm.get(&p1.1).unwrap().clone().1 + hm.get(&p1.1).unwrap().clone().0 * hm.get(&p1.0).unwrap().clone().1;
                let bot = hm.get(&p1.0).unwrap().clone().1 * hm.get(&p1.1).unwrap().clone().1 * 2;
                hm.insert(hp.clone(),(top, bot));
            }
        }
    }


}

fn read_line() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string().split(" ").map(|x| x.to_string()).collect()
}