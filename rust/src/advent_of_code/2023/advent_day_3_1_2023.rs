use std::io;

fn main(){
    let mut total;
    let mut matrix = Vec::new();

    loop{
        let input = read_input();
        let input: Vec<_> = input.chars().collect();
        matrix.push(input);

        total = calc_parts(&matrix);
        println!("total = {}", total);
    }
}


fn calc_parts(matrix:&Vec<Vec<char>>) -> i64 {
    let mut total = 0;
    for row in 0..matrix.len() {
        let mut t = 0;
        let mut start:i64 = -1;
        let mut end:i64 = -1;
        for col in 0..matrix[row].len() {
            if matrix[row][col].is_digit(10) {
                if start < 0 {
                    start = col as i64;
                }
                end = col as i64;
                t = t * 10 + matrix[row][col].to_digit(10).unwrap() as i64;
            } 
            if (!matrix[row][col].is_digit(10) || col+1 == matrix[row].len()) && t != 0 {
                let b = symbol_next(row, start as usize, end as usize, &matrix);
                if b {
                    total += t;
                }
                t = 0;
                start = -1;
                end = -1;
            }
        }
    }

    total
}


fn symbol_next(row:usize, start:usize, end:usize, matrix:&Vec<Vec<char>>) -> bool{
    let row_start = if row != 0 { row-1 } else { row };
    let row_end = if row+1 != matrix.len() { row+2 } else { row+1 };
    let col_start = if start != 0 { start-1 } else { start };
    let col_end = if end+1 != matrix[0].len() { end+2 } else { end+1 };

    for i in row_start..row_end {
        for j in col_start..col_end {
            if !matrix[i][j].is_digit(10) && matrix[i][j] != '.' {
                return true;
            } 
        }
    }
    false
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}