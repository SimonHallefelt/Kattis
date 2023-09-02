use std::io;

fn main(){
    let n = read_line()[0];
    let input = read_line();
    let mut max = 0_f64;

    let mut sum_r = 0;
    let mut sum_l = 0;
    for i in 0..n as usize {
        sum_r += input[i];
        sum_l += input[(n-1-i as i128) as usize];
        let temp_r = sum_r as f64 / (i+1)as f64;
        let temp_l = sum_l as f64 / (i+1) as f64;
        if temp_r > max {
            max = temp_r;
        }
        if temp_l > max {
            max = temp_l;
        }
    }
    println!("{}", max);

}

fn read_line() -> Vec<i128>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("no input");
    input.trim().to_string().split(" ").map(|x| x.parse::<i128>().unwrap()).collect()
}