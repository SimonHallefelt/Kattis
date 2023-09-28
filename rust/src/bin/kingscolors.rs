use std::io;

fn main () {
    let input = read_input();
    for _ in 0..input[0]-1 {read_input();}
    println!("{}", run(input[0], input[1]));
}

fn run(n: i128, k: i128) -> i128 {
    let mut v: Vec<Vec<i128>> = Vec::new();
    let max: i128 = n+1;
    let modd = 1000000007;
    let mut factorial = Vec::new();
    factorial.push(1);
    for i in 0..max {
        let temp = Vec::new();
        v.push(temp);
        factorial.push((factorial[i as usize]*(i+1)) % modd)
    }
    //k*(k-1)^(n-1)-k*(k-1)(k-2)^(n-1)+
    for n in n..max {
        for k in 0..k+1 {
            if n < 2 || k < 2 {
                v[n as usize].push(1);
                continue;
            } else if k == 2 {
                v[n as usize].push(2);
                continue;
            }
            let mut aw = k;
            for _ in 1..n {
                aw = (aw*(k-1)) % modd;
            }
            let mut nacu = k *(k-1);
            for _ in 1..n {
                nacu = (nacu*(k-2)) % modd;
            }
            let mut fac = 1;
            //println!("");
            for i in (2..k).rev() {
                //fac = factorial[k as usize] / ((factorial[i as usize] * factorial[(k-i) as usize]) % modd);
                let fac_t = factorial[k as usize];
                let fac_b = (factorial[i as usize] * factorial[(k-i) as usize]) % modd;
                fac = fac_t / fac_b; //blir fel vid stora k på grund av mod (tror jag)
                
                //print!("i = {}, fac = {} ", i, fac);
                aw = (aw - fac * v[n as usize][(i) as usize]) % modd;
                if aw < 0 {aw += modd}
            }
            v[n as usize].push(aw.clone());
        }
        //println!("");
        //println!("{:?}", v[n as usize]);
    }
   v[n as usize][k as usize]
}

fn read_input() -> Vec<i128> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string().split(" ").map(|x| x.parse::<i128>().unwrap()).collect()
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
        assert_eq!(answer, 1200); //1200 kanske
    }

    #[test]
    fn test_speed() {
        let start = SystemTime::now();
        let answer = run(2500,2500);
        let done = SystemTime::now();
        let time = done.duration_since(start);
        assert_eq!(answer, -1, "time = {:?}", time); //time = 0,901 ms
    }
}