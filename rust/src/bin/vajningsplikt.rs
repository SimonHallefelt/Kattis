use std::{io, collections::HashMap};

fn main(){
    let input = readline();
    let input: Vec<&str> = input.trim().split(" ").collect();

    let mut map: HashMap<&str, i32> = HashMap::new();
    map.insert("South", 0);
    map.insert("West", 1);
    map.insert("North", 2);
    map.insert("East", 3);

    if (map.get(&input[0]).unwrap() + 2) % 4 == *map.get(&input[1]).unwrap() {
        if (map.get(&input[0]).unwrap() + 3) % 4 == *map.get(&input[2]).unwrap(){
            println!("Yes");
        }else {
            println!("No");
        }
    }else if (map.get(&input[0]).unwrap() + 1) % 4 == *map.get(&input[1]).unwrap() {
        if (map.get(&input[0]).unwrap() + 1) % 4 == *map.get(&input[2]).unwrap(){
            println!("No");
        }else {
            println!("Yes");
        }
    }else {
        println!("No");
    }
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}