use std::io;

fn main(){
    let input = read_line();
    let l = input[0];
    let mut w = input[1];
    if l > w || l*26 < w {
        println!("impossible");
        return;
    }
    let mut letters: Vec<String> = Vec::new();
    for i in 65..91 {
        let letter = i as u8 as char;
        letters.push(letter.to_lowercase().to_string());
    }
    for i in 0..l {
        let mut temp = w/(l-i);
        if temp > 26 {
            temp = 26;
        }
        w -= temp;
        print!("{}", letters[(temp-1) as usize]);
    }
}

fn read_line() -> Vec<i64>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().split(" ").map(|x| x.parse::<i64>().unwrap()).collect()
}