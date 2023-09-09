use std::io;

fn main(){
    let input = read_line();
    let g = input[0];
    let s = input[1];
    let c = input[2];
    let b = g*3+s*2+c;
    if b >= 8 {
        print!("Province or ");
    }else if b >= 5 {
        print!("Duchy or ");
    }else if b >= 2 {
        print!("Estate or ");
    }
    if b >= 6 {
        println!("Gold");
    }else if b >= 3 {
        println!("Silver");
    }else {
        println!("Copper");
    }
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}