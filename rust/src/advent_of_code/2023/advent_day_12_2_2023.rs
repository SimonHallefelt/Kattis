use std::{fs, collections::HashMap};

fn main(){
    let mut total: i64 = 0;
    
    //let file_path = "src\\advent_of_code\\2023\\data\\day_12_test.txt"; //525152
    let file_path = "src\\advent_of_code\\2023\\data\\day_12.txt"; //280382734828319
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        let data = fix_data(input.0);
        let damages = input.1;
        let mut hm: HashMap<(usize, usize), i64> = HashMap::new();
        
        //println!("{:?}", data);

        //total += damage_combinations(data, damages);
        let t = dp(&data, &damages, 0, 0, &mut hm);
        total += t;
        print!("{} ", t);
    }
    println!("");

    println!("total = {}", total);
}


fn dp(data: &Vec<usize>, damages: &Vec<usize>, p: usize, d: usize, hm: &mut HashMap<(usize, usize), i64>) -> i64 {
    let mut combinations = 0;
    if hm.contains_key(&(p,d)) {return *hm.get(&(p,d)).unwrap();}
    for i in p..data.len()+1-damages[d] {
        if data[i] == 1 {
            let mut b = fits(data, damages, i, d);
            if !b {break;}

            //får sättas in?
            if d+1 == damages.len() {
                for j in i+damages[d]..data.len() {
                    if data[j] == 1 {b = false; break;}
                }
                if !b {break;}
                combinations += 1;
            } else if i+damages[d] < data.len()+1-damages[d+1] {
                combinations += dp(data, damages, i+damages[d]+1, d+1, hm);
            }
            break;
        } else if data[i] == 2 {
            let mut b = fits(data, damages, i, d);
            if !b {continue;}

            if d+1 == damages.len() {
                for j in i+damages[d]..data.len() {
                    if data[j] == 1 {b = false; break;}
                }
                if !b {continue;}
                combinations += 1;
            } else if i+damages[d] < data.len()+1-damages[d+1] {
                combinations += dp(data, damages, i+damages[d]+1, d+1, hm);
            }
        }
    }
    hm.insert((p,d), combinations);
    combinations
}


fn fits(data: &Vec<usize>, damages: &Vec<usize>, i: usize, d: usize) -> bool {
    for j in i+1..i+damages[d] {
        if data[j] == 0 {return false}
    }
    if i+damages[d] < data.len() && data[i+damages[d]] == 1 {return false}
    true
}


fn fix_data(data: Vec<char>) -> Vec<usize> {
    let mut d = Vec::new();
    let mut last = false;
    for c in data {
        if !(c == '.' && last) {
            match c {
                '.' => {d.push(0); last = true},
                '#' => {d.push(1); last = false},
                _ => {d.push(2); last = false}
            }
        }
    }
    d
}


fn fix_input(line: &str) -> (Vec<char>, Vec<usize>) {
    let parts: Vec<&str> = line.trim().split(" ").collect();
    if parts.len() != 2 {
        println!("{:?}", parts);
    }

    let mut part_0 = Vec::new();
    let mut part_1 = Vec::new();
    for _ in 0..5 {
        part_0.push(parts[0].trim().to_string().clone());
        part_1.push(parts[1].trim().to_string().clone());
    }
    let data = part_0.join("?");
    let damages = part_1.join(",");

    let data2 = data.trim().chars().collect();
    let damages2 = damages.trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect();

    (data2, damages2)
}

