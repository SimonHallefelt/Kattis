use std::{cmp::{max, min}, collections::HashMap, io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let n = stdin.lock().lines().take(1).nth(0).unwrap().unwrap().trim().parse().unwrap();

    let mut data = Vec::with_capacity(n);
    for line in stdin.lock().lines().take(n) {
        let row: Vec<i32> = line.unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (x, y) = (row[0], row[1]);
        data.push((x, y));
    }

    let result = run(&data);
    if result.0 {
        println!("learning is guaranteed!")
    } else {
        println!("{} {}", result.1.0+1, result.1.1+1)
    }
}

fn run(data: &Vec<(i32, i32)>) -> (bool, (usize, usize)) {
    let mut hm: HashMap<(i32, i32), Vec<(usize, usize)>> = HashMap::new();
    // for i in 0..data.len()-1 {
        let (x1, y1) = data[0];
        for j in 1..data.len() {
            let (x2, y2) = data[j];
            // (x1-x2)*a + (y1-y2)*b = 0, find all pairs a and b
            let xd = x1-x2; 
            let yd = y1-y2;
            if xd == 0 {
                // b must be 0, a can be anything
                if hm.contains_key(&(1, 0)) {
                    hm.get_mut(&(1, 0)).unwrap().push((0, j));
                } else {
                    hm.insert((1, 0), vec![(0, j)]);
                }
            } 
            if yd == 0 {
                // a must be 0, b can be anything
                if hm.contains_key(&(0, 1)) {
                    hm.get_mut(&(0, 1)).unwrap().push((0, j));
                } else {
                    hm.insert((0, 1), vec![(0, j)]);
                }
            } 
            if xd != 0 && yd != 0 {
                // there is a pattern that can be calculated, xd*a = yd*b, gcd(xd, yd)
                let gcd = gcd(xd.abs(), yd.abs());
                let mut t = -1;
                if min(xd, yd) < 0 && max(xd, yd) > 0 {
                    t = 1;
                }
                let tilt = (yd.abs()/gcd, (xd.abs()/gcd)*t);
                if hm.contains_key(&tilt) {
                    hm.get_mut(&tilt).unwrap().push((0, j));
                } else {
                    hm.insert(tilt, vec![(0, j)]);
                }
            }
            // there are no more possible patterns
        }
    // }

    for i in hm {
        if i.1.len() == 1 {
            return (false, i.1[0]);
        }
    }

    (true, (0, 0))
}


fn gcd(mut n: i32, mut m: i32) -> i32 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
          std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let data = vec![(0,0), (0,1), (1,0)];
        let result = run(&data);
        assert!(!result.0);
        assert_eq!(result.1, (1-1, 2-1))
    }
}