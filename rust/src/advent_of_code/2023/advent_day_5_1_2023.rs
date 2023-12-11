use std::io;

fn main(){
    let mut total;
    let seeds: Vec<i64> = read_input().split(":")
        .nth(1).unwrap().trim().split(" ")
        .map(|x| x.parse::<i64>().unwrap()).collect();
    
    let mut matrix = Vec::new();
    loop {
        let input = read_input();
        if input.contains(":") {
            matrix.push(Vec::new());
        }else if input.len() > 0 {
            let m = matrix.len();
            matrix[m-1].push(input.split(" ").map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>());
        }
        if matrix.len() > 6 {
            break;
        }
    }

    loop{
        let input = read_input();
        let m = matrix.len();
        matrix[m-1].push(input.split(" ").map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<i64>>());

        total = get_min_seed(&matrix, &seeds);

        println!("total = {}", total);
    }
}


fn get_min_seed(matrix: &Vec<Vec<Vec<i64>>>, seeds: &Vec<i64>) -> i64{
    let mut locations = Vec::new();
    for i in 0..seeds.len() {
        let mut value = seeds[i];
        for j in 0..7 {
            for r in &matrix[j] {
                if value >= r[1] && value < r[1]+r[2] {
                    value = r[0] + (value - r[1]);
                    break;
                }
            }
        }
        locations.push(value)
    }
    *locations.iter().min().unwrap()
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}