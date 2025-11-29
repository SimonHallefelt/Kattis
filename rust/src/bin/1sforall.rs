use std::io;

fn main() {
    let s = read_line();
    let mut v = run(s);
    println!("{}", v);
}

fn run(s: String) -> i32 {
    let mut v = 0;
    let mut numbers: Vec<u32> = s.chars().map(|a| a.to_digit(10).unwrap()).collect();
    for i in 0..numbers.len()-1 {
        let a = numbers[i];
        let b = numbers[i+1];
        if b == 0 {
            numbers[i+1] = 10;
            if a == 1 {
                numbers[i] = 0;
            }
        }
    }
    for d in numbers {
        match d {
            1 => v += 1,
            2 => v += 2,
            3 => v += 3,
            4 => v += 4,
            5 => v += 5,
            6 => v += 5,
            7 => v += 6,
            8 => v += 6,
            9 => v += 6,
            10 => v += 7,
            _ => v += 0,
        }
    }
    v
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, run("1".to_string()));
        assert_eq!(3, run("12".to_string()));
        assert_eq!(12, run("77".to_string()));
        assert_eq!(2, run("11".to_string()));
        assert_eq!(9, run("20".to_string()));
        assert_eq!(7, run("10".to_string()));
        assert_eq!(14, run("100".to_string()));
        assert_eq!(5*7, run("100000".to_string()));
        assert_eq!(1+1+3+3, run("99".to_string()));
        assert_eq!(5, run("24".to_string()));
        assert_eq!(5+5, run("2424".to_string()));
        assert_eq!(1+1+2+2+2+1, run("89".to_string()));
    }
}