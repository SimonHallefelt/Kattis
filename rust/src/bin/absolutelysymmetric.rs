use std::io;

fn main(){
    let n = read_line().parse::<i128>().unwrap();
    let mut v = Vec::new();
    for _ in 0..n {
        let input = read_line();
        let input: Vec<i128> = input.split_whitespace().map(|x| x.parse::<i128>().unwrap()).collect();
        v.push(input);
    }
    run(n, v);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}



fn run(n: i128, v: Vec<Vec<i128>>) {
    let mut matrix_mid: Vec<Vec<i128>> = vec![vec![0; n as usize]; n as usize];
    let mut matrix_ajust: Vec<Vec<i128>> = vec![vec![0; n as usize]; n as usize];
    let mut adjust = false;
    for i in 0..n {
        for j in 0..n {
            if v[i as usize][j as usize].abs() == v[j as usize][i as usize].abs() {
                matrix_mid[i as usize][j as usize] = v[i as usize][j as usize];
                matrix_mid[j as usize][i as usize] = v[j as usize][i as usize];
            }else if (v[i as usize][j as usize] - v[j as usize][i as usize])%2 == 0 {
                adjust = true;
                let mid = (v[i as usize][j as usize] + v[j as usize][i as usize])/2;
                matrix_mid[i as usize][j as usize] = mid;
                matrix_mid[j as usize][i as usize] = mid;
                matrix_ajust[i as usize][j as usize] = v[i as usize][j as usize] - mid;
                matrix_ajust[j as usize][i as usize] = v[j as usize][i as usize] - mid;
            }else {
                println!("-1");
                return;
            }
        }
    }

    if !adjust {
        println!("1") 
    }else {
        println!("2");
        for i in 0..n {
            println!("{}", matrix_ajust[i as usize].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
        }
    }
    for i in 0..n {
        println!("{}", matrix_mid[i as usize].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}