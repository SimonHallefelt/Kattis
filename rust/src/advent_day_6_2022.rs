use std::io;
use std::collections::HashSet;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let s = s.trim();

    let cha: Vec<char> = s.chars().collect();

    for i in 0..cha.len()-14{
        let j = i as usize;
        let mut set = HashSet::new();

        for k in i..i+14 {
            set.insert(cha[k]);
        }

        if set.len() == 14 {
            println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", cha[j], cha[j+1], cha[j+2], cha[j+3], cha[j+4], cha[j+5], cha[j+6], cha[j+7], cha[j+8], cha[j+9], cha[j+10], cha[j+11], cha[j+12], cha[j+13]);
            println!("i = {}", i+14);
            break;
        }
    }
}


