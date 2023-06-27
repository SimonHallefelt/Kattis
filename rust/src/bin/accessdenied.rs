use std::{io, process::exit};

fn main(){
    let mut number = 0;
    let mut out = "".to_string();

    for i in 1..21 {
        out += "A";
        println!("{}", out);
        let response = read_input();
        if response.trim().to_string() == "ACCESS GRANTED".trim().to_string() {
            exit(0);
        }
        if response.trim().to_string() != "ACCESS DENIED (5 ms)" {
            number = i;
            break;
        }
    }

    let mut awnser = "".to_string();
    for pos in 0..number {
        let leter: String = correct_leter(pos, number, awnser.clone());
        awnser += &leter.clone();
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn correct_leter(pos: i32, number: i32, awnser: String) -> String{
    let mut leter: String = "-".to_string();
    let pass = get_pass();
    loop{
        let mut out = awnser.clone();
        for _ in pos..number {
            out += "A";
        }
        leter = next_leter(leter);
        out.replace_range(pos as usize..(pos + 1) as usize, &leter.clone());
        println!("{}", out);

        let response = read_input();
        if response.trim().to_string() == "ACCESS GRANTED".trim().to_string() {
            exit(0);
        }
        for i in (pos+1) as usize..pass.len() {
            if response.trim().to_string() == pass[i].trim().to_string() {
                return leter;
            }
        }
    }
}

fn next_leter(leter: String) -> String {
    let leters = "-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".to_string();
    let leters: Vec<char> = leters.chars().collect();
    for i in 0..leters.len() {
        if leters[i].to_string() == leter {
            return leters[i+1].to_string();
        }
    }
    return "a".to_string();
}

fn get_pass() -> Vec<String> {
    let mut pass: Vec<String> = Vec::new();
    pass.push("ACCESS DENIED (5 ms)".to_string());
    for i in 1..22 {
        let temp = (14 + 9 * i).to_string();
        let mut s = "ACCESS DENIED (".to_string();
        s += temp.as_str();
        s += &" ms)".to_string();
        pass.push(s.clone());
    }

    return pass;
}