use std::{io, collections::HashMap};

fn main() {
    let mut right = 0;
    let mut answer: i128 = 0;
    let mut wrongs = HashMap::new();

    loop {
        let input = read_line();
        if input == "-1" {
            println!("{} {}", right, answer);
            break;
        }
        let input: Vec<String> = input.split(" ").map(|x| x.to_string()).collect();

        if input[2] == "wrong" {
            if wrongs.contains_key(&input[1]) {
                let wrong = wrongs.get(&input[1]).unwrap() + 1;
                wrongs.insert(input[1].clone(), wrong);
            }else {
                wrongs.insert(input[1].clone(), 1);
            }
        }else {
            right += 1;
            answer += input[0].parse::<i128>().unwrap();
            if wrongs.contains_key(&input[1]) {
                answer += wrongs.get(&input[1]).unwrap() * 20
            }
        }
    }
}

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}