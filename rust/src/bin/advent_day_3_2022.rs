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
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().to_string();
        let le = input.chars().count();

        let mut set = HashSet::new();

        for i in 0..(le/2)-1{
            let temp1 = input.chars().nth(i).unwrap().to_string();
            set.insert(temp1);
        }
        for i in (le/2)-1..le{
            if set.contains(input.chars().nth(i).unwrap().to_string().as_str()){
                total += point_holder[input.chars().nth(i).unwrap().to_string().as_str()];
                break;
            }
        }
        println!("total = {}", total);
    }

}

