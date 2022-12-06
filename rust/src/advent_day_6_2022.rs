use std::io;

fn main() {
    let s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let c: Vec<char> = s.chars().collect();

    for i in 0..c.len(){
        
    }

}


