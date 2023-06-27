use std::io;
use std::collections::HashMap;

fn main(){
    let n: i128 = read_line().parse::<i128>().unwrap();
    for i in 0..n {
        let input: String = read_line();
        let input: Vec<&str> = input.trim().split(" ").collect();
        translate(i, input[0].to_string(), input[1].to_string(), input[2].to_string());
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn translate(case: i128, s1: String, s2: String, s3: String){
    let mut value: i128 = 0;
    let mut awnser: String = String::new();
    let mut from: HashMap<char, i128> = HashMap::new();

    for c in s2.chars(){
        from.insert(c, value);
        value += 1;
    } value = 0;

    let mut multi = s1.len();
    for c in s1.chars(){
        multi -= 1;
        value += from.get(&c).unwrap() * (s2.len().pow(multi as u32)) as i128;
    }

    while value > 0 {
        let rest = value % s3.len() as i128;
        value = value / s3.len() as i128;
        awnser = s3.chars().nth(rest as usize).unwrap().to_string() + &awnser;
    }

    println!("Case #{}: {} ", case+1, awnser)
}
