use std::io;

fn main() {
    let mut matrix_input: Vec<Vec<i32>> = Vec::new();
    let mut matrix_score: Vec<Vec<i32>> = Vec::new();

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let input: Vec<char> = input.chars().collect();

        let mut vec_i = Vec::new();
        let mut vec_b = Vec::new();
        for i in 0..input.len(){
            vec_i.push(input[i].to_string().parse::<i32>().unwrap());
            vec_b.push(0);
        }

        matrix_input.push(vec_i);
        matrix_score.push(vec_b);

        //reset matrix_score
        for i in 0..matrix_score.len(){
            for j in 0..matrix_score[i].len(){
                matrix_score[i][j] = 0;
            }
        }


        for a in 1..matrix_input.len()-1{
            for b in 1..matrix_input[a].len()-1{
                let mut free_right = 0;
                let mut free_left = 0;
                let mut free_up = 0;
                let mut free_down = 0;
                let height = matrix_input[a][b];

                for i in b+1..matrix_input[a].len(){
                    free_right += 1;
                    if height <= matrix_input[a][i] {
                        break;
                    }
                }
                
                for i in 0..b{
                    free_left += 1;
                    if height <= matrix_input[a][b-1-i] {
                        break;
                    }
                }
                
                for i in 0..a{
                    free_up += 1;
                    if height <= matrix_input[a-1-i][b] {
                        break;
                    }
                }
                
                for i in a+1..matrix_input.len(){
                    free_down += 1;
                    if height <= matrix_input[i][b] {
                        break;
                    }
                }
                
                matrix_score[a][b] = free_left * free_right * free_up * free_down;
            }
        }

        let mut max = 0;
        for i in 0..matrix_score.len(){
            for j in 0..matrix_score[i].len(){
                if max < matrix_score[i][j] {
                    max = matrix_score[i][j];
                }
            }
        }
        println!("max: {}", max);
    }
}
