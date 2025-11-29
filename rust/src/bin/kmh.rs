use std::{io};

fn main(){
    let n = read_line_stdin().parse().unwrap();

    let mut m = 0;
    for _ in 0..n {
        let input = read_line_stdin();
        if input != "/" {
            println!("{}", input);
            let v = (input.parse::<i32>().unwrap()/10)*10+10;
            if v > m {
                m = v;
            }
        } else {
            println!("{}", m)
        }
    }
}

fn read_line_stdin() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}