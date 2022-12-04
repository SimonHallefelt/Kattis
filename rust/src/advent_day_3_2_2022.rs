use std::io;
use std::collections::HashMap;
use std::collections::HashSet;

fn main(){
    let alp1: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
    let alp2: Vec<&str> = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];

    let mut point_holder: HashMap<String, i32> = HashMap::new();

    for i in 0..26{
        let temp: i32 = i as i32;
        point_holder.insert(alp1[i].to_string(), temp+1);
    }
    for i in 0..26{
        let temp: i32 = i as i32;
        point_holder.insert(alp2[i].to_string(), temp+1+26);
    }

    let mut total: i32 = 0;

    loop{
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();

        io::stdin().read_line(&mut input1).expect("Failed to read line");
        io::stdin().read_line(&mut input2).expect("Failed to read line");
        io::stdin().read_line(&mut input3).expect("Failed to read line");
        input1.trim().to_string();
        input2.trim().to_string();
        input3.trim().to_string();
        
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();

        for i in 0..input1.chars().count(){
            let temp = input1.chars().nth(i).unwrap().to_string();
            set1.insert(temp);
        }
        for i in 0..input2.chars().count(){
            if set1.contains(input2.chars().nth(i).unwrap().to_string().as_str()){
                let temp = input2.chars().nth(i).unwrap().to_string();
                set2.insert(temp);
            }
        }
        for i in 0..input3.chars().count(){
            if set2.contains(input3.chars().nth(i).unwrap().to_string().as_str()){
                total += point_holder[input3.chars().nth(i).unwrap().to_string().as_str()];
                break;
            }
        }
        println!("total = {}", total);
    }
}

