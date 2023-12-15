use std::io;

fn main () {
    let mut inputt = io::stdin().lines().into_iter();
    loop {
        let r: Result<String, std::io::Error> = Result::Ok("$".to_string());
        let mut input = inputt.next().unwrap_or(r).unwrap_or("$".to_string());
        let mut n = 0;
        if input == "$" {
            break;
        }
        n = input.parse::<i128>().unwrap();
        let mut str = Vec::new();
        for _ in 0..n {
            input = inputt.next().unwrap().unwrap();
            str.push(input.clone());
        }
        input = inputt.next().unwrap().unwrap();
        let sent: Vec<char> = input.chars().collect();

        let br: i128 = 200;
        let prim: i128 = (10 as i128).pow(9)+7;
        for s in str {
            let mut v = Vec::new();
            let mut hash: i128 = 0;
            let temp: Vec<char> = s.chars().collect();
            for i in 0..s.len(){
                hash = (temp[i] as u8 as i128 + (hash * br)) % prim;
            }
            let mut st: i128 = 0;
            for i in 0..s.len()-1{
                st = (sent[i] as u8 as i128 + (st * br)) % prim;
            }
            for i in s.len()-1..sent.len() {
                st = (sent[i] as u8 as i128 + (st * br)) % prim;
                if st == hash {
                    v.push(i-(s.len()-1));
                }
                st = (st - sent[i-(s.len()-1)] as u8 as i128 * br.pow(((s.len() as i128)-1) as u32))%prim;
                if st < 0 {
                    st = st + prim;
                }
            }
            
            for i in v {
                print!("{} ", i)
            }
            println!()
        }
    }
}