use std::io;

fn main() {
    let input = readline();
    let input: Vec<&str> = input.trim().split(" ").collect();
    let row = input[0].parse::<i128>().unwrap();
    let col = input[1].parse::<i128>().unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for _ in 0..row {
        let input = readline();
        let row: Vec<char> = input.trim().chars().collect();
        matrix.push(row);
    }
    run(matrix, row, col);
}

fn run(mut matrix: Vec<Vec<char>>, row: i128, col: i128) {
    for i in 0..(row-1) as usize {
        let mut changed = true;
        while changed {
            changed = false;
            for j in 0..col as usize {
                if matrix[i][j] == 'V' {
                    if matrix[i +1][j] == '.'{
                        matrix[i +1][j] = 'V';
                    }else if matrix[i +1][j] == '#' {
                        if j > 0 && matrix[i][j -1] == '.' {
                            matrix[i][j -1] = 'V';
                            changed = true;
                        }
                        if j+1 < col as usize && matrix[i][j +1] == '.' {
                            matrix[i][j +1] = 'V';
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    for i in 0..row as usize {
        for j in 0..col as usize {
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}

fn readline() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}