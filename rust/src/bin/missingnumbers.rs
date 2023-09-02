use std::io;

fn main(){
    let n = read_line();
    let mut m = Vec::new();
    let mut a = 0;
    for _ in 0..n{
        let b = read_line();
        if a+1 != b{
            for i in a+1..b {
                println!("{}", i);
                m.push(b);
            }
        }
        a=b;
    }
    if m.len() == 0{
        println!("good job");
    }
}

fn read_line() -> i64{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().parse::<i64>().unwrap()
}