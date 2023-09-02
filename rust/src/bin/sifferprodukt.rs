use std::io;

fn main() {
    let mut number: String = readline();
    while number.len() > 1 {
        let mut sum: i128 = 1;
        for c in number.chars() {
            let digit = c.to_digit(10).unwrap() as i128;
            if digit != 0 {
                sum *= digit;
            }
        }
        number.clear();
        number.push_str(&sum.to_string());
    }
    println!("{}", number);
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}