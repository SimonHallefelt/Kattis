

fn main(){
    let n: i64 = read_input().parse().unwrap();
    
}

fn read_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    return input;
}