use std::{io, collections::HashMap};

fn main () {
    let input = read_input();
    for _ in 0..input[0]-1 {read_input();}
    println!("{}", run(input[0], input[1]));
}

fn run(n: i64, k: i64) -> i64 {
    let mut hm = HashMap::new();
    let modd = 1000000007;
    for n in 0..n+1 {
        hm.insert((n, 2), 1);
        hm.insert((n, n), 1);
    }
    let mut factorial = Vec::new();
    factorial.push(1);
    for i in 0..k+1 {
        factorial.push((factorial[i as usize]*(i+1)) % modd)
    }
    run2(n, k, &mut hm, modd) * factorial[k as usize] % modd
}

fn run2(n: i64, k: i64, hm: &mut HashMap<(i64, i64), i64>, modd: i64) -> i64 {
    if hm.contains_key(&(n, k)) {
        return *hm.get(&(n, k)).unwrap();
    }
    let temp1 = run2(n-1, k, hm, modd);
    let temp2 = run2(n-1, k-1, hm, modd);
    let answer = (temp1 * (k-1) + temp2) % modd;
    hm.insert((n, k), answer);
    answer
}

fn read_input() -> Vec<i64> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}


//tests
#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use super::*;

    #[test]
    fn sample_input_1() {
        let answer = run(4,3);
        assert_eq!(answer, 18);
    }

    #[test]
    fn sample_input_2() {
        let answer = run(6,4);
        assert_eq!(answer, 600);
    }

    #[test]
    fn test_1() {
        let answer = run(6,5);
        assert_eq!(answer, 1200);
    }

    #[test]
    fn test_speed_1() {
        let start = SystemTime::now();
        let answer = run(2500,2500);
        let done = SystemTime::now();
        let time = done.duration_since(start);
        assert_eq!(answer, 954730329, "time = {:?}", time);
    }

    #[test]
    fn test_speed_2() {
        let start = SystemTime::now();
        let answer = run(2500,1250);
        let done = SystemTime::now();
        let time = done.duration_since(start);
        assert_eq!(answer, 99379097, "time = {:?}", time);
    }
    //time 2,5s
}