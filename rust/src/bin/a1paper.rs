use std::{io};

fn main(){
    read_input();
    let input = read_input();
    let papers: Vec<i128> = input.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
    run(papers);
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}



fn run(papers: Vec<i128>) {
    let target = 2i128.pow(30);
    let paper_size: Vec<i128> = get_paper_size();
    let tape_amounts: Vec<f64> = get_tape_amounts();
    let mut area: i128 = 0;
    let mut tape: f64 = 0.0;

    for i in 0..papers.len() {
        if area + paper_size[i] * papers[i] > target {
            let n = (target - area) / paper_size[i];
            area += paper_size[i] * n;
            tape += tape_amounts[i+1] * n as f64;
            break;
        } else {
            area += paper_size[i] * papers[i];
            tape += tape_amounts[i+1] * papers[i] as f64;
        }

        if area == target {
            break;
        }
    }
    if area < target {
        println!("impossible");
        return;
    }
    println!("{}", (tape - tape_amounts[0])/ 2 as f64)
}

fn get_paper_size() -> Vec<i128> {
    let mut paper_size: Vec<i128> = Vec::new();
    let mut a = 2i128.pow(30);
    for _ in 0..30 {
        a = a/2;
        paper_size.push(a);
    }
    paper_size
}

fn get_tape_amounts() -> Vec<f64> {
    let mut tape_amounts: Vec<f64> = Vec::new();
    let mut a1: f64 = 2f64.powf(-5 as f64/4 as f64) * 2 as f64;
    let mut a2: f64 = 2f64.powf(-3 as f64 / 4 as f64);
    tape_amounts.push(a1*2 as f64 + a2*2 as f64);
    for i in 0..30 {
        if i%2 == 0 {
            a1 = a1/2f64;
        } else {
            a2 = a2/2f64;
        }
        tape_amounts.push(a1*2 as f64 + a2*2 as f64);
    }
    tape_amounts
}