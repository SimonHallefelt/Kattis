use std::{io, string};

fn main(){
    loop {
        let input = read();
        if input[0].clone() == "0" {
            break;
        }
        run(input);
    }
}

fn run(case: Vec<String>){
    let player1 = pos(case[0].clone(), case[1].clone());
    let player2 = pos(case[2].clone(), case[3].clone());

    let mut win = 0;
    let mut total = 0;
    for x in player1 {
        for y in player2.clone() {
            total += 1;
            if x>y {
                win += 1;
            }
        }
    }

    if win == total {
        println!("1");
    }else if win == 0{
        println!("0")
    }else {
        loop {
            let gcd = gcd(win, total);
            if gcd == 1 {
                break;
            }
            win = win/gcd;
            total = total/gcd;
        }
        println!("{}/{}", win, total)
    }
}

fn pos(s1: String, s2: String) -> Vec<i64>{
    let mut v = Vec::new();
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    if s1 == "*" {
        for i in 1..7{
            v1.push(i);
        }
    }else{
        v1.push(s1.parse::<i64>().unwrap())
    }
    if s2 == "*" {
        for i in 1..7{
            v2.push(i);
        }
    }else{
        v2.push(s2.parse::<i64>().unwrap())
    }
    let mut temp = Vec::new();
    for x in &v1 {
        for y in &v2 {
            temp.push((x.clone(), y.clone()))
        }
    }
    for (x, y) in temp {
        if x==y {
            v.push((x+y)*1000)
        }else if x+y == 3 {
            v.push(1000000)
        }else {
            if x>y{
                v.push(x*10+y)
            }else{
                v.push(y*10+x)
            }
        }
    }
    v
}

fn read() -> Vec<String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    input.split(" ").map(|x| x.to_string()).collect()
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