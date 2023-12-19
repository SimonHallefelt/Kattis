use std::{fs, collections::{HashMap, hash_map}};

fn main(){
    let mut total: i128 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_19_test.txt"; //167409079868000
    let file_path = "src\\advent_of_code\\2023\\data\\day_19.txt"; //134906204068564
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map = HashMap::new();
    for c in contents.trim().split("\n") {
        if c.trim().len() == 0 {
            break;
        }
        let input = fix_input_map(c);
        map.insert(input.0, input.1);
    }

    total += run(&map);

    println!("total = {}", total);
}


fn run(map: &HashMap<String, Vec<(char, bool, i128, String)>>) -> i128 {
    let mut pos = "in".to_string();
    let mut tot = 0;
    let mut queue = Vec::new();
    let start_ranges = HashMap::
        from([('x', (1, 4000)), ('m', (1, 4000)), ('a', (1, 4000)), ('s', (1, 4000))]);
    queue.push((map.get(&pos).unwrap(), start_ranges));

    while !queue.is_empty() {
        let temp = queue.pop().unwrap();
        let conditions = temp.0;
        let mut ranges = temp.1;
        for i in 0..conditions.len() {
            let c = &conditions[i];
            let new_ranges;
            if i+1 == conditions.len() {
                new_ranges = ranges.clone();
            } else {
                let mut ranges_1 = HashMap::new();
                let mut ranges_2 = HashMap::new();

                for r in ranges.clone() {
                    if r.0 == c.0 {
                        if c.1 {
                            if c.2 < r.1.0 || c.2 >= r.1.1 {
                                ranges_1.insert(r.0, r.1);
                                ranges_2.insert(r.0, r.1);
                            } else {
                                ranges_1.insert(r.0, (r.1.0, c.2));
                                ranges_2.insert(r.0, (c.2+1, r.1.1));
                            }
                        } else {
                            if c.2 <= r.1.0 || c.2 > r.1.1 {
                                ranges_1.insert(r.0, r.1);
                                ranges_2.insert(r.0, r.1);
                            } else {
                                ranges_1.insert(r.0, (c.2, r.1.1));
                                ranges_2.insert(r.0, (r.1.0, c.2-1));
                            }
                        }
                    } else {
                        ranges_1.insert(r.0, r.1);
                        ranges_2.insert(r.0, r.1);
                    }
                }
                ranges = ranges_1;
                new_ranges = ranges_2;
            }
            if c.3 == "A".to_string() {
                let mut t = 1;

                for hs in new_ranges.clone() {
                    t *= 1+hs.1.1-hs.1.0;
                }

                tot += t;
            } else if c.3 != "R".to_string() {
                queue.push((map.get(&c.3).unwrap(), new_ranges.clone()));
            }

        }
    }

    tot
}


fn fix_input_map(line: &str) -> (String, Vec<(char, bool, i128, String)>) {
    let temp: Vec<String> = line.trim().split("{").map(|x| x.to_string()).collect();
    let name = temp[0].clone();
    let mut conditions = Vec::new();
    for input in temp[1].split(",") {
        if input.contains("}") {
            conditions.push((' ', false, 0, input.trim().replace("}", "")));
        } else {
            let i: Vec<char> = input.clone().chars().collect();
            let var = i[0];
            let mut larger_than = true;
            if '<' == i[1] {
                larger_than = false;
            }
            let mut num = 0;
            for j in 2..i.len() {
                if ':' == i[j] {
                    break;
                }
                num = num * 10 + i[j].to_digit(10).unwrap() as i128; 
            }
            let to = input.trim().split(":").nth(1).unwrap().to_string();
            conditions.push((var, larger_than, num, to));
        }

    }
    (name, conditions)
}
