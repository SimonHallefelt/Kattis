use std::io;

fn main() {
    let mut matrix_visited: Vec<Vec<String>> = Vec::new();
    let mut visited: i32 = 1;
    let mut rope: Vec<Vec<i32>> = Vec::new();
    
    for _ in 0..10 {
        let mut temp_vec: Vec<i32> = Vec::new();
        temp_vec.push(500);
        temp_vec.push(500);
        rope.push(temp_vec);
    }

    for _ in 0..1000 {
        let mut temp_vec: Vec<String> = Vec::new();
        for _ in 0..1000 {
            temp_vec.push(".".to_string());
        }
        matrix_visited.push(temp_vec);
    }

    matrix_visited[500 as usize][500 as usize] = "#".to_string();

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let input: Vec<&str> = input.split(" ").collect();
        //println!("input = {:?}", input)

        for _ in 0..input[1].to_string().parse::<i32>().unwrap() {
            if input[0].to_string() == "D" {
                rope[0][0] += 1;
            }else if input[0].to_string() == "U" {
                rope[0][0] -= 1;
            }else if input[0].to_string() == "R" {
                rope[0][1] += 1;
            }else{ //down
                rope[0][1] -= 1;
            }

            for i in 0..9 {
                if (rope[i][0] - rope[i+1][0]).abs() == 2 {
                    rope[i+1][0] += (rope[i][0] - rope[i+1][0])/2;
                    if (rope[i][1] - rope[i+1][1]).abs() == 1 {
                        rope[i+1][1] += rope[i][1] - rope[i+1][1];
                    }else if (rope[i][1] - rope[i+1][1]).abs() == 2 {
                        rope[i+1][1] += (rope[i][1] - rope[i+1][1])/2;
                    }
                }else if (rope[i][1] - rope[i+1][1]).abs() == 2 {
                    rope[i+1][1] += (rope[i][1] - rope[i+1][1])/2;
                    if (rope[i][0] - rope[i+1][0]).abs() == 1 {
                        rope[i+1][0] += rope[i][0] - rope[i+1][0];
                    }else if (rope[i][0] - rope[i+1][0]).abs() == 2 {
                        rope[i+1][0] += (rope[i][0] - rope[i+1][0])/2;
                    }
                }
            }

            if matrix_visited[rope[9][0] as usize][rope[9][1] as usize] == "." {
                matrix_visited[rope[9][0] as usize][rope[9][1] as usize] = "#".to_string();
                visited += 1;
            }
        }
        println!("visited = {}", visited);
        /* if visited == 36 {
            for i in 0..matrix_visited.len(){
                println!("{:?}", matrix_visited[i]);
            }
        } */

        //temp
        /* let mut temp1: Vec<Vec<String>> = Vec::new();
        for _ in 0..12 {
            let mut temp_vec: Vec<String> = Vec::new();
            for _ in 0..12 {
                temp_vec.push(".".to_string());
            }
            temp1.push(temp_vec);
        }

        for i in 0..10 {
            temp1[rope[9-i][0] as usize][rope[9-i][1] as usize] = (9-i).to_string();
        }

        for i in 0..temp1.len(){
            println!("{:?}", temp1[i]);
        } */

    }
}
