use std::io;

fn main(){
    let input = read_line();
    let n = input[0];
    let m = input[1];

    run(m, n, 1);
}

fn run(m:i64, n:i64, a:i64){
    let g = gcd(m, n);
    if m == n {
        println!("{}", m*a);
    }else if m%2 == n%2 && m%2 == 1{
        println!("{}", g*a);
    }else if m%2 == n%2{
        run(m/2, n/2, a*2);
    }else {
        println!("{}", 0);
    }
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no read");
    let input = input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
    input
}

pub fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
}