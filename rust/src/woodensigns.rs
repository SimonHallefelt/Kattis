use std::io;

fn main(){
    let n: i64 = read_input().parse().unwrap();
    let vec: Vec<i64> = read_input().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; (n+2) as usize]; (n+2) as usize];
    
    println!("{}", solve(&vec, &mut dp, vec[0], vec[1], 2));
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_string();
    return input;
}

fn solve(vec: &Vec<i64>, dp: &mut Vec<Vec<i64>>, l: i64, r: i64, steps: i64) -> i64 {
    let modulo: i64 = 2147483647;
    if steps >= vec.len() as i64 {
        return 1;
    }

    if dp[l as usize][r as usize] != 0 {
        return dp[l as usize][r as usize];
    }

    let step = vec[steps as usize];
    if step < l {
        dp[l as usize][r as usize] = solve(vec, dp, step, r, steps+1);
        return dp[l as usize][r as usize];
    }else if r < step {
        dp[l as usize][r as usize] = solve(vec, dp, l, step, steps+1);
        return dp[l as usize][r as usize];
    }else {
        dp[l as usize][r as usize] = (solve(vec, dp, l, step, steps+1) + solve(vec, dp, step, r, steps+1)) % modulo;
        return dp[l as usize][r as usize];
    }
}