use std::{io, collections::{HashMap, HashSet}};

fn main(){
    let st = read_line();
    let ch: Vec<char> = st.chars().collect();
    let mut map = HashMap::new();
    for c in &ch {
        if map.contains_key(&c) {
            let temp = *map.get(&c).unwrap();
            map.insert(c, temp+1);
        }else{
            map.insert(c, 1);
        }
    }

    let keys = map.keys();
    let mut max: i64 = 0;
    for i in 0..ch.len() {
        let mut set = HashSet::new();
        for j in i+1..ch.len() {
            if ch[i] == ch[j] {
                break;
            }
            if !set.contains(&ch[j]){
                max += 1;
                set.insert(ch[j]);
                if set.len() == keys.len()-1{
                    break;
                }
            } 
        }
    }
    println!("{}", max)
}

fn read_line() -> String{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string()
}