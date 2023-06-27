use std::io;

fn main(){
    let n  = read_line().parse::<i128>().unwrap();
    if n % 2 == 0 {
        println!("Bob");
    }else{
        println!("Alice");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}