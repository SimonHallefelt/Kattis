use std::{io, collections::{HashSet, HashMap}};

fn main(){
    let input: String = read_line();
    let n = input.to_string().parse::<i128>().unwrap();
    let input: String = read_line();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let mut v: Vec<i128> = Vec::new();
    for i in 0..n {
        v.push(input[i as usize].to_string().parse::<i128>().unwrap());
    }
    let map = get_map(n);
    println!("{}", play(n, v, map));
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn get_map(n: i128) -> HashMap<i128, HashSet<i128>>{
    let mut map: HashMap<i128, HashSet<i128>> = HashMap::new();
    for i in 0..n {
        map.insert(i, HashSet::new());
    }
    return map;
}



fn play(n: i128, v: Vec<i128>, mut map: HashMap<i128, HashSet<i128>>) -> i128{
    let mut awnser = 0;
    for i in 0..n {
        let mut pos = i;
        let mut moves: Vec<i128> = Vec::new();
        let mut unique: HashSet<i128> = HashSet::new();
        let mut vis: HashMap<i128, i128> = HashMap::new();
        let mut set = HashSet::new();
        let vec = v.clone();

        moves.push(pos);
        vis.insert(v[pos as usize], 1);
        unique.insert(v[pos as usize]);
        loop {
            set.insert(pos);
            pos += v[pos as usize];
            moves.push(pos);
            if pos >= 0 && pos < n {
                if vis.contains_key(&v[pos as usize]){
                    vis.insert(v[pos as usize], vis.get(&v[pos as usize]).unwrap() + 1);
                } else {
                    vis.insert(v[pos as usize], 1);
                }
                unique.insert(v[pos as usize]);
            }
            if set.contains(&pos){
                cycle(vec, &mut map, &mut moves, pos, vis, unique);
                break;
            } else if pos < 0 || pos >= n || map.get(&pos).unwrap().len() != 0{
                unique = HashSet::new();
                if map.contains_key(&pos){
                    unique = map.get(&pos).unwrap().clone();
                }
                back_trace(vec, &mut map, &mut moves, unique);
                break;
            }
        }
        awnser += map.get(&i).unwrap().len() as i128;
    }

    return awnser;
}

fn cycle(v: Vec<i128>, map: &mut HashMap<i128, HashSet<i128>>, moves: &mut Vec<i128>, 
            pos: i128, mut vis: HashMap<i128, i128>, mut unique: HashSet<i128>){
    let mut p;
    let mut cycle = false;
    for i in 0..moves.len() {
        p = moves[i];
        if map.contains_key(&p){
            if cycle {
                map.insert(moves[i], unique.clone());
            }else {
                if p == pos {
                    cycle = true;
                }else{
                    map.insert(p, unique.clone());
                    vis.insert(v[p as usize], vis.get(&v[p as usize]).unwrap() - 1);
                    if vis.get(&v[p as usize]).unwrap() == &0 {
                        unique.remove(&p);
                    }
                }
            }
        }
    }
}

fn back_trace(v: Vec<i128>, map: &mut HashMap<i128, HashSet<i128>>, moves: &mut Vec<i128>, 
                            mut unique: HashSet<i128>){
    for i in (0..moves.len()).rev() {
        if map.contains_key(&moves[i]){
            unique.insert(v[moves[i] as usize]);
            map.insert(moves[i], unique.clone());
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bara_höger() {
        let n = 4;
        let v = vec![1, 2, 3, 4];
        let map = get_map(n);
        let awnser = play(n, v, map);
        assert_eq!(awnser, 7);
    }

    #[test]
    fn cirkulär() {
        let n = 2;
        let v = vec![1, -1];
        let map = get_map(n);
        let awnser = play(n, v, map);
        assert_eq!(awnser, 4);
    }

    #[test]
    fn inte_räkna_upprepade_siffror() {
        let n = 3;
        let v = vec![1, 1, 1];
        let map = get_map(n);
        let awnser = play(n, v, map);
        assert_eq!(awnser, 3);
    }

    #[test]
    fn bara_höger_cirkulär() {
        let n = 3;
        let v = vec![1, 1, -1];
        let map = get_map(n);
        let awnser = play(n, v, map);
        assert_eq!(awnser, 6);
    }

    #[test]
    fn bara_vänster() {
        let n = 4;
        let v = vec![-4, -3, -2, -1];
        let map = get_map(n);
        let awnser = play(n, v, map);
        assert_eq!(awnser, 7);
    }
}
        