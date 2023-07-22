use std::{io, collections::HashMap};

fn main(){
    let input = readline();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let q = input[1].parse::<i128>().unwrap();
    let mut queries = Vec::new();
    
    for _ in 0..q {
        let input = readline();
        let query: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
        queries.push(query);
    }

    run(queries);
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn run(queries: Vec<Vec<String>>) {
    let mut set_values: HashMap<i128, i128> = HashMap::new();
    let mut default_value: i128 = 0;

    for querie in queries {
        if querie[0] == "PRINT".to_string() {
            if set_values.contains_key(&querie[1].parse::<i128>().unwrap()){
                println!("{}", set_values.get(&querie[1].parse::<i128>().unwrap()).unwrap());
            }else {
                println!("{}", default_value);
            }
        } else if querie[0] == "RESTART".to_string() {
            set_values = HashMap::new();
            default_value = querie[1].parse::<i128>().unwrap();
        } else {
            set_values.insert(querie[1].parse::<i128>().unwrap(), querie[2].parse::<i128>().unwrap());
        }
    }
}