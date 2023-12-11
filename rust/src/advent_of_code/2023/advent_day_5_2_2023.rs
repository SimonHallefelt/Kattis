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

        for i in 0..m {
            matrix[i].sort_by(|a, b| a[1].cmp(&b[1]));
        }

        total = find_location(&matrix, &seeds);

        println!("total = {}", total); //7873084
    }
}


fn find_location(matrix: &Vec<Vec<Vec<i64>>>, seeds: &Vec<i64>) -> i64{
    let mut location = std::i64::MAX;
    for i in 0..seeds.len()/2 {
        let mut values = Vec::new();
        values.push((seeds[i*2], seeds[i*2]+seeds[i*2+1]-1));
        for j in 0..7 {
            let mut new_values = Vec::new();
            for mut v in values {
                for r in &matrix[j] {
                    if v.1 < r[1] {
                        new_values.push(v);
                        break;
                    } else if v.0 < r[1]+r[2] {
                        if v.0 < r[1] {
                            new_values.push((v.0, r[1]-1));
                            v.0 = r[1];
                        }
                        let v0 = r[0] + v.0.max(r[1]) - r[1];
                        let v1 = r[0] + v.1.min(r[1]+r[2]-1) - r[1];
                        new_values.push((v0, v1));
                        if v.1 < r[1]+r[2] {
                            break;
                        }
                        v.0 = r[1]+r[2]
                    }
                }
                if v.1 >= &matrix[j][&matrix[j].len()-1][1] + &matrix[j][&matrix[j].len()-1][2] {
                    new_values.push(v);
                }
            }
            values = new_values;
        }
        location = location.min(values.iter().map(|x| x.0).min().unwrap());
    }
    location
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}