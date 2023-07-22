use std::{io, collections::HashMap};

fn main(){
    let n = read_line().parse::<i128>().unwrap();

    for _ in 0..n {
        run();
    }
}

fn run(){
    let n = read_line().parse::<i128>().unwrap();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut vec = Vec::new();

    for _ in 0..n {
        let input = read_line();
        let input = input.split(" ").map(|x| x.to_string()).collect::<Vec<String>>();
        let name: String = input[0].clone()[0..(input[0].len()-1)].to_string();
        let class = input[1].clone();
        let class = class.split("-").map(|x| x.to_string()).collect::<Vec<String>>();

        let mut alphabetic_class = String::new();
        for c in class {
            if c == "upper" {
                alphabetic_class = "A".to_string() + &alphabetic_class
            }else if c == "middle" {
                alphabetic_class = "B".to_string() + &alphabetic_class
            }else {
                alphabetic_class = "C".to_string() + &alphabetic_class
            }
        }
        for _ in alphabetic_class.len()..10 {
            alphabetic_class.push('B');
        }

        if map.contains_key(&alphabetic_class) {
            let mut temp = map.get(&alphabetic_class).unwrap().clone();
            temp.push(name.clone());
            map.insert(alphabetic_class.clone(), temp);
        }else {
            let v = vec![name.clone()];
            map.insert(alphabetic_class.clone(), v);
            vec.push(alphabetic_class.clone());
        }
    }

    vec.sort();
    for s in vec {
        let mut v = map.get(&s).unwrap().clone();
        v.sort();
        for name in v {
            println!("{} ", name);
        }
    }
    for _ in 0..30 {
        print!("=");
    }
    println!("");
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

