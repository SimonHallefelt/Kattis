use std::{fs, collections::HashMap};

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_19_test.txt"; //19114
    let file_path = "src\\advent_of_code\\2023\\data\\day_19.txt"; //401674
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = HashMap::new();
    let mut map_done = false;
    for c in contents.trim().split("\n") {
        if c.trim().len() == 0 {
            map_done = true;
            continue;
        }
        if !map_done{
            let input = fix_input_map(c);
            map.insert(input.0, input.1);
        } else {
            let input = fix_input_part(c);
            total += run(&map, input);
        }

    }


    println!("total = {}", total);
}


fn run(map: &HashMap<String, Vec<(String, bool, i64, String)>>, input: HashMap<String, i64>) -> i64 {
    let mut pos = "in".to_string();
    loop {
        let conditions = map.get(&pos).unwrap();
        for i in 0..conditions.len() {
            let c = &conditions[i];
            if i+1 == conditions.len() {
                pos = c.3.clone();
                break;
            }
            if c.1 && c.2 < *input.get(&c.0).unwrap() {
                pos = c.3.clone();
                break;
            } else if !c.1 && c.2 > *input.get(&c.0).unwrap() {
                pos = c.3.clone();
                break;
            }
        }
        if pos == "A".to_string() {
            break;
        }
        if pos == "R".to_string() {
            return 0;
        }
    }

    let mut tot = 0;
    for v in input {
        tot += v.1;
    }
    tot
}


fn fix_input_part(line: &str) -> HashMap<String, i64> {
    let mut hs = HashMap::new();
    let input: Vec<String> = line.trim()
    .replace("{", "").replace("}", "")
    .split(",").map(|x| x.to_string()).collect();
    for s in input {
        let temp: Vec<String> = s.split("=").map(|x| x.to_string()).collect();
        hs.insert(temp[0].clone(), temp[1].parse::<i64>().unwrap());
    }
    hs
}


fn fix_input_map(line: &str) -> (String, Vec<(String, bool, i64, String)>) {
    let temp: Vec<String> = line.trim().split("{").map(|x| x.to_string()).collect();
    let name = temp[0].clone();
    let mut conditions = Vec::new();
    for input in temp[1].split(",") {
        if input.contains("}") {
            conditions.push(("".to_string(), false, 0, input.trim().replace("}", "")));
        } else {
            let i: Vec<char> = input.clone().chars().collect();
            let var = i[0].to_string();
            let mut larger_than = true;
            if '<' == i[1] {
                larger_than = false;
            }
            let mut num = 0;
            for j in 2..i.len() {
                if ':' == i[j] {
                    break;
                }
                num = num * 10 + i[j].to_digit(10).unwrap() as i64; 
            }
            let to = input.trim().split(":").nth(1).unwrap().to_string();
            conditions.push((var, larger_than, num, to));
        }

    }
    (name, conditions)
}
