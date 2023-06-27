use std::io;

fn main(){
    let n = read_line().parse::<i128>().unwrap();
    let mut querys = Vec::new();
    for _ in 0..n {
        let input = read_line();
        let input: Vec<i128> = input.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
        querys.push(input);
    }
    run(querys);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}



fn run(querys: Vec<Vec<i128>>){
    for query in querys {
        let mut unique = 18;
        for _ in 0..(query[0] + query[1] - 2) {
            unique = (unique * 6)%(10i128.pow(9) +7);
        }
        for _ in 0..((query[0] - 1)*(query[1] - 1)) {
            unique = (unique * 2)%(10i128.pow(9) +7);
        }
        println!("{}", unique);
    }
}