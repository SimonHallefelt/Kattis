use std::{io, cmp};

fn main(){
    let mut total: i32 = 0;
    loop{
        let mut input = read_line();
        let chars: Vec<char> = input.chars().collect();
        let mut min: i32 = -1;
        let mut max: i32 = -1; 
        for i in 0..chars.len() {
            if chars[i].is_digit(10) {
                if min < 0 {
                    min = chars[i].to_digit(10).unwrap() as i32;
                }
                max = chars[i].to_digit(10).unwrap() as i32;
            }
            else{
                to_word(&mut min, &mut max, chars.clone(), i);
            }
        }
        total += min*10+max;
        println!("{}", total);
    }

}

fn to_word( min:&mut i32, max:&mut i32, chars:Vec<char>, i:usize){
    let mut words: Vec<Vec<char>> = Vec::new();
    words.push("one".chars().collect());
    words.push("two".chars().collect());
    words.push("three".chars().collect());
    words.push("four".chars().collect());
    words.push("five".chars().collect());
    words.push("six".chars().collect());
    words.push("seven".chars().collect());
    words.push("eight".chars().collect());
    words.push("nine".chars().collect());

    for k in 0..words.len(){
        let w = words[k].clone();
        let mut b = true;
        if chars.len() >= i+w.len() {
            for j in 0..w.len() {
                if w[j] != chars[i+j] {
                    b = false;
                }
            }
        }else{b=false}
        if b {
            if min.clone() < 0 {
                *min = (k+1) as i32;
            }
            *max = (k+1) as i32;
            return;
        }
        
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}