use std::{fs, collections::HashMap};

fn main(){
    let mut total: i64 = 0;
    //let file_path = "D:\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_8_test.txt";
    //let file_path = "D:\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_8_test2.txt";
    let file_path = "D:\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_8.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let mut lr: Vec<_>= Vec::new();
    let mut map = HashMap::new();

    //println!("{}", contents);
    for c in contents.split("\n").enumerate() {
        if c.1.trim().len() == 0 {continue;}
        else if c.0 == 0 {lr = c.1.trim().chars().collect();}
        else {
            let s: Vec<&str> = c.1.trim().split(" ").collect();
            let k = s[0].to_string();
            let v = (s[2].trim().replace("(", "").replace(",", ""), 
                                        s[3].trim().replace(")", ""));
            map.insert(k, v);
        }
    }

    let mut found = false;
    let mut pos = "AAA".to_string();
    let mut movee = 0;
    while !found {
        if lr[movee] == 'L' {pos = map.get(&pos).unwrap().0.clone()}
        else {pos = map.get(&pos).unwrap().1.clone()}

        total += 1;
        movee += 1; movee %= lr.len();
        if pos == "ZZZ" {found = true}
    }

    println!("total = {}", total);
}
