use std::io;

fn main() {
    let mut matrix_visited: Vec<Vec<bool>> = Vec::new();
    let mut visited: i32 = 1;
    let mut pos_h_x: i32 = 500;
    let mut pos_h_y: i32 = 500;
    let mut pos_t_x: i32 = 500;
    let mut pos_t_y: i32 = 500;

    for _ in 0..1000 {
        let mut temp_vec: Vec<bool> = Vec::new();
        for _ in 0..1000 {
            temp_vec.push(false);
        }
        matrix_visited.push(temp_vec);
    }

    matrix_visited[pos_t_x as usize][pos_t_y as usize] = true;

    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input = input.trim().to_string();
        let input: Vec<&str> = input.split(" ").collect();
        //println!("input = {:?}", input)

        for _ in 0..input[1].to_string().parse::<i32>().unwrap() {
            if input[0].to_string() == "R" {
                pos_h_x += 1;
            }else if input[0].to_string() == "L" {
                pos_h_x -= 1;
            }else if input[0].to_string() == "U" {
                pos_h_y += 1;
            }else{ //down
                pos_h_y -= 1;
            }

            if (pos_h_x - pos_t_x).abs() == 2 {
                pos_t_x += (pos_h_x - pos_t_x)/2;
                if (pos_h_y - pos_t_y).abs() == 1 {
                    pos_t_y += pos_h_y - pos_t_y;
                }
            }else if (pos_h_y - pos_t_y).abs() == 2 {
                pos_t_y += (pos_h_y - pos_t_y)/2;
                if (pos_h_x - pos_t_x).abs() == 1 {
                    pos_t_x += pos_h_x - pos_t_x;
                }
            }

            if !matrix_visited[pos_t_x as usize][pos_t_y as usize] {
                matrix_visited[pos_t_x as usize][pos_t_y as usize] = true;
                visited += 1;
            }
        }
        println!("pos_h_x = {}, pos_h_y = {}", pos_h_x, pos_h_y);
        println!("pos_t_x = {}, pos_t_y = {}", pos_t_x, pos_t_y);
        println!("visited = {}", visited);
    }
}
