use std::io;


fn main(){
    let mut grid = [[0,0,0,0],
                                    [0,0,0,0],
                                    [0,0,0,0],
                                    [0,0,0,0]];

    for i in 0..4 {
        let input: String = read_line();
        let input: Vec<&str> = input.trim().split(" ").collect();
        for j in 0..4 {
            grid[i][j] = input[j].to_string().parse::<i128>().unwrap();
        }
    }
    let direction = read_line().parse::<i128>().unwrap();

    if direction == 0 {
        left(&mut grid);
    } else if direction == 1 {
        up(&mut grid);
    } else if direction == 2 {
        right(&mut grid);
    } else if direction == 3 {
        down(&mut grid);
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn left(grid: &mut [[i128; 4]; 4]){
    for k in 0..4 {
        for i in 0..4 {
            let mut v1 = 0;
            for j in i..4 {
                if grid[k][j] != 0 {
                    if v1 == 0 {
                        v1 = grid[k][j];
                        grid[k][j] = 0;
                    }else if v1 == grid[k][j] {
                        v1 = v1 * 2;
                        grid[k][j] = 0;
                        break;
                    }else {
                        break;
                    }
                }
            }
            grid[k][i] = v1;
        }
    }
    print_grid(grid);
}

fn up(grid: &mut [[i128; 4]; 4]){
    for k in 0..4 {
        for i in 0..4 {
            let mut v1 = 0;
            for j in i..4 {
                if grid[j][k] != 0 {
                    if v1 == 0 {
                        v1 = grid[j][k];
                        grid[j][k] = 0;
                    }else if v1 == grid[j][k] {
                        v1 = v1 * 2;
                        grid[j][k] = 0;
                        break;
                    }else {
                        break;
                    }
                }
            }
            grid[i][k] = v1;
        }
    }
    print_grid(grid);
}

fn right(grid: &mut [[i128; 4]; 4]){
    for k in 0..4 {
        for i in (0..4).rev() {
            let mut v1 = 0;
            for j in (0..i+1).rev() {
                if grid[k][j] != 0 {
                    if v1 == 0 {
                        v1 = grid[k][j];
                        grid[k][j] = 0;
                    }else if v1 == grid[k][j] {
                        v1 = v1 * 2;
                        grid[k][j] = 0;
                        break;
                    }else {
                        break;
                    }
                }
            }
            grid[k][i] = v1;
        }
    }
    print_grid(grid);
}

fn down(grid: &mut [[i128; 4]; 4]){
    for k in 0..4 {
        for i in (0..4).rev() {
            let mut v1 = 0;
            for j in (0..i+1).rev() {
                if grid[j][k] != 0 {
                    if v1 == 0 {
                        v1 = grid[j][k];
                        grid[j][k] = 0;
                    }else if v1 == grid[j][k] {
                        v1 = v1 * 2;
                        grid[j][k] = 0;
                        break;
                    }else {
                        break;
                    }
                }
            }
            grid[i][k] = v1;
        }
    }
    print_grid(grid);
}

fn print_grid(grid: &mut [[i128; 4]; 4]){
    for i in 0..4 {
        for j in 0..4 {
            print!("{} ", grid[i][j]);
        }
        println!("");
    }
}