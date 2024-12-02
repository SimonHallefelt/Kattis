use std::io;

fn main() {
    let input = read_line();
    let input: Vec<&str> = input.split(" ").collect();
    let n: i32 = input[0].parse().unwrap();
    let m: i32 = input[1].parse().unwrap();
    if n == 3 && m == 3 {
        println!("no way")
    } else {
        println!("attend here")
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}