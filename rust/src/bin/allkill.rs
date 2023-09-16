use std::{io, collections::HashSet};

fn main() {
    let input = read_line();
    let n = input[0];
    let t = input[1];
    let mut v = Vec::new();
    for _ in 0..n {
        v.push(read_line()[0]);
    }

    let p_finish = 0; 
        



    let p_switch = 0;




    let p = p_finish*p_switch;
    println!("{}", (p*(t.pow(n as u32))) % 998244353 )
}

fn _probability_to_finish(round: i128, v: Vec<(i128, i128)>, s: &mut HashSet<Vec<(i128, i128)>>) -> f64{
    if v.is_empty() {
        return 1.0;
    }
    let mut p = 0.0;
    let mut temp = 0;
    for i in 0..v.len() {
        temp += v[i].1;
    }
    if temp > round {
        return 0.0;
    }
    let pr = 1 as f64 / round as f64;
    let mut s1 = HashSet::new();
    s.insert(v.clone());
    p += (1 as f64 - pr).powf(v.len() as f64) * _probability_to_finish(round - 1, v.clone(), &mut s1); // alla misslyckade
    for i in 0..v.len() {
        let mut v2 = v.clone();
        v2.remove(i);
        if !s.contains(&v2) {
            s.insert(v2.clone());
            p += pr * _probability_to_finish(round, v2.clone(), s); // en lyckad
        }
    }
    p
}

fn _probability_to_not_switch() -> f64 {
    1.0
}

fn read_line() -> Vec<i128> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    input.trim().to_string().split(" ").map(|x| x.parse::<i128>().unwrap()).collect()
}


//tests
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_probability_to_not_switch() {
        let n = 3;
        let t: i128 = 5;
        let mut v = Vec::new();
        v.push((1, 1));
        v.push((2, 2));
        v.push((3, 1));
        let mut s = HashSet::new();
        let p_finish = _probability_to_finish(t, v, &mut s);
        let p_switch = _probability_to_not_switch(); // 5.0/6.0
        let p = p_finish*p_switch;
        let answer = (p*(t.pow(n as u32)) as f64) as i64 % 998244353;
        println!("p = {}, answer = {} ", p, answer);
        assert_eq!(answer, 60);
    }

    #[test]
    fn test_probability_to_finish() {
        let n = 5;
        let t: i128 = 5;
        let mut v = Vec::new();
        v.push((1, 1));
        v.push((2, 1));
        v.push((3, 1));
        v.push((4, 1));
        v.push((5, 1));
        let mut s = HashSet::new();
        let p_finish = _probability_to_finish(t, v, &mut s);
        let p_switch = 1.0; //fix
        let p = p_finish*p_switch;
        let answer = (p*(t.pow(n as u32)) as f64) as i64 % 998244353;
        println!("p = {}, answer = {} ", p, answer);
        assert_eq!(answer, 1296);
    }
}