use std::io;
use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::LinkedList;

fn main() {
    let mut pos: Vec<String> = Vec::new();
    let mut set: Vec<String> = Vec::new();
    let mut map_s: HashMap<String, HashSet<String>> = HashMap::new();
    let mut map_i: HashMap<String, i32>  = HashMap::new();

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_string();
        let vec: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();

        if vec[0] =="$" {
            if vec[1] == "cd" {
                if vec[2] == ".."{
                    pos.pop();
                }else{
                    let i = set.len();
                    if !set.is_empty(){
                        let mut temp = map_s.insert(pos[pos.len()-1].to_string(), HashSet::new()).unwrap();
                        temp.insert(i.to_string());
                        map_s.insert(pos[pos.len()-1].to_string(), temp);
                    }
                    pos.push(i.to_string());
                    set.push(i.to_string());
                    map_s.insert(i.to_string(), HashSet::new());
                    map_i.insert(i.to_string(), 0);
                }
            }
        }else {
            if vec[0] != "dir" {
                let mut temp: i32 = map_i.insert(pos[pos.len()-1].to_string(), 0).unwrap();
                temp += vec[0].to_string().parse::<i32>().unwrap();
                map_i.insert(pos[pos.len()-1].to_string(), temp);
            }
        }

        let mut total: i32 = 0;
        for s in &set{
            let mut temp1: i32 = 0;
            let mut temp2: LinkedList<String> = LinkedList::new();
            temp2.push_back(s.to_string());
            while !temp2.is_empty() && temp1 <= 100000{
                let temp3: String = temp2.pop_front().unwrap();
                temp1 += map_i.get(&temp3).unwrap().to_owned();
                let temp3: HashSet<String> = map_s.get(&temp3).unwrap().to_owned();

                for ss in &temp3{
                    if set.contains(ss) {
                        temp2.push_back(ss.to_string());
                    }
                }
            }

            if temp1 <= 100000 {
                total += temp1;
            }
        }
        println!("total = {}", total);
    }
}
