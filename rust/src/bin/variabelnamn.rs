use std::{io, collections::{HashSet, HashMap}};

fn main() {
    let n = readline().parse::<i128>().unwrap();
    let mut set: HashSet<i128> = HashSet::new();
    let mut map: HashMap<i128, i128> = HashMap::new();
    for _ in 0..n {
        let m = readline().parse::<i128>().unwrap();
        let mut i = 1;
        if map.contains_key(&m) {
            *map.get_mut(&m).unwrap() += 1;
            i = *map.get(&m).unwrap();
        }
        loop{
            if set.contains(&(m*i)) {
                i += 1;
            } else {
                set.insert(m*i);
                map.insert(m, i);
                println!("{}", m*i);
                break;
            }
        }

    }
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}