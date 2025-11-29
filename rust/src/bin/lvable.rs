use std::io;

fn main() {
    read_line();
    let s = read_line();
    if s.contains("lv") {
        println!("0");
    } else if s.contains("v") || s.contains("l") {
        println!("1");
    } else {
        println!("2");
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}
