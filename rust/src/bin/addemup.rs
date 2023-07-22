use std::{io, collections::HashSet};

fn main(){
    let input = read_line();
    let input: Vec<i128> = input.split(" ").map(|x| x.parse::<i128>().unwrap()).collect();
    let target = input[1];

    let numbers = read_line();
    run(numbers.clone(), target);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn run(numbers: String, target: i128) -> String{
    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for s in numbers.split(" ") {
        if !set2.contains(&s.to_string().parse::<i128>().unwrap()) {
            let mut v = Vec::new();
            v.push(s.to_string().parse::<i128>().unwrap());
            let mut temp = String::new();
            if !(s.contains("3") || s.contains("4") || s.contains("7")) {
                for c in s.chars() {
                    if c == '6' {
                        temp.push('9');
                    }else if c == '9' {
                        temp.push('6');
                    }else { // 0, 1, 2, 5, 8
                        temp.push(c);
                    }
                }
                temp = temp.chars().rev().collect::<String>();
                v.push(temp.parse::<i128>().unwrap());
            }
            for n in v.clone() {
                let need = target - n;
                if set1.contains(&need){
                    println!("YES");
                    return "YES".to_string();
                }
            }
            for n in v {
                if !set1.contains(&n) {
                    set1.insert(n);
                } else {
                    set2.insert(n);
                }
            }
        }
    }
    println!("NO");
    return "NO".to_string();
}



// tests
#[cfg(test)]
mod tests {
    use super::*;
    // byt plats och snurra 180 grader
    // 1 2 3 4 5 6 7 8 9 0 = 1 2 3 4 5 9 7 8 6 0

    #[test]
    fn test1() {
        let target = 66;
        let numbers = "15 21 22".to_string();
        assert_eq!(run(numbers.clone(), target), "NO"); //51+15=66 
    }

    #[test]
    fn test2() {
        let target = 63;
        let numbers = "15 21 22".to_string();
        assert_eq!(run(numbers.clone(), target), "YES");//12+51=63
    }
}