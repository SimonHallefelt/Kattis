use std::io;

fn main(){
    let mut total;
    let mut matrix = Vec::new();

    loop{
        let input = read_input();
        let input: Vec<_> = input.chars().collect();
        matrix.push(clean_input(input));

        total = find_symbol(&matrix);
        println!("total = {}", total);
    }
}


fn clean_input(input:Vec<char>) -> Vec<String> {
    let mut new_input = Vec::new();
    let mut t = 0;
    let mut start:i64 = -1;
    let mut end:i64 = -1;
    for col in 0..input.len() {
        if input[col].is_digit(10) {
            if start < 0 {
                start = col as i64;
            }
            end = col as i64;
            t = t * 10 + input[col].to_digit(10).unwrap() as i64;
        } 
        if (!input[col].is_digit(10) || col+1 == input.len()) && t != 0 {
            for _ in start..end+1 {
                new_input.push(t.to_string());
            }
            t = 0;
            start = -1;
            end = -1;
        }
        if !input[col].is_digit(10) {
            if input[col] == '*' {
                new_input.push("*".to_string())
            }else {
                new_input.push(".".to_string())
            }
        }
    }
    
    new_input
}


fn find_symbol(matrix:&Vec<Vec<String>>) -> i64{
    let mut total = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == "*" {
                total += is_gear(matrix, i, j);
            } 
        }
    }
    
    total
}

fn is_gear(matrix:&Vec<Vec<String>>, row:usize, col:usize) -> i64 {
    let row_start = if row != 0 { row-1 } else { row };
    let row_end = if row+1 != matrix.len() { row+2 } else { row+1 };
    let col_start = if col != 0 { col-1 } else { col };
    let col_end = if col+1 != matrix[0].len() { col+2 } else { col+1 };
    let mut numbers = Vec::new();

    for i in row_start..row_end {
        let mut found = false;
        for j in col_start..col_end {
            if matrix[i][j] != "*" && matrix[i][j] != "." {
                if !found {
                    found = true;
                    numbers.push(matrix[i][j].parse::<i64>().unwrap());
                }
            } else {
                found = false;
            }
        }
    }

    if numbers.len() == 2 {
        return numbers[0] * numbers[1]
    }
    0
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}