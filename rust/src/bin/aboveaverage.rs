use std::io;

fn main() {
    let c: i128 = read_input().parse::<i128>().unwrap();
    let mut querys: Vec<Vec<i128>> = Vec::new();
    for _ in 0..c {
        let input = read_input();
        let input: Vec<i128> = input.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
        querys.push(input);
    }
    run(querys);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    return input.trim().to_string();
}



fn run(querys: Vec<Vec<i128>>) {
    for query in querys {
        let mut avrage = 0;
        for i in 1..query[0]+1 {
            avrage += query[i as usize];
        }
        avrage = avrage / query[0];

        let mut above_avrage: i128 = 0;
        for i in 1..query[0]+1 {
            if query[i as usize] > avrage {
                above_avrage += 1;
            }
        }

        let result = above_avrage as f64 * 100 as f64 / query[0] as f64;
        println!("{:.3}%", result)
    }
}