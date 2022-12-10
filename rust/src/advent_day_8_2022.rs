use std::io;

fn main() {
    let mut matrix_input: Vec<Vec<i32>> = Vec::new();
    let mut matrix_can_see: Vec<Vec<bool>> = Vec::new();

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let input: Vec<char> = input.chars().collect();
        println!("input = {:?}", input);

        let mut vec_i = Vec::new();
        let mut vec_b = Vec::new();
        for i in 0..input.len(){
            vec_i.push(input[i].to_string().parse::<i32>().unwrap());
            vec_b.push(false);
        }

        matrix_input.push(vec_i);
        matrix_can_see.push(vec_b);

        //reset matrix_can_see
        for i in 0..matrix_can_see.len(){
            for j in 0..matrix_can_see[i].len(){
                matrix_can_see[i][j] = false;
            }
        }

        let mut left = -1;
        let mut right = -1;
        for i in 0..matrix_input.len(){
            for j in 0..matrix_input[i].len(){
                if left < matrix_input[i][j] {
                    left = matrix_input[i][j];
                    matrix_can_see[i][j] = true;
                }
                if right < matrix_input[i][matrix_input[i].len()-1-j] {
                    right = matrix_input[i][matrix_input[i].len()-1-j];
                    matrix_can_see[i][matrix_input[i].len()-1-j] = true;
                }
            }
            left = -1;
            right = -1;
        }
        let mut up = -1;
        let mut down = -1;
        for i in 0..matrix_input[0].len(){
            for j in 0..matrix_input.len(){
                if up < matrix_input[j][i] {
                    up = matrix_input[j][i];
                    matrix_can_see[j][i] = true;
                }
                if down < matrix_input[matrix_input.len()-1-j][i] {
                    down = matrix_input[matrix_input.len()-1-j][i];
                    matrix_can_see[matrix_input.len()-1-j][i] = true;
                }
            }
            up = -1;
            down = -1;
        }

        let mut total = 0;
        for i in 0..matrix_can_see.len(){
            for j in 0..matrix_can_see[i].len(){
                if matrix_can_see[i][j] {
                    total += 1;
                }
            }
        }
        //println!("matrix_input = {:?}", matrix_input);
        //println!("matrix_can_see = {:?}", matrix_can_see);
        println!("total: {}", total);
    }
}
