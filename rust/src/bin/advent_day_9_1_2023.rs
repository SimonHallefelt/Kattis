use std::fs;

fn main(){
    let mut total: i64 = 0;
    
    //let file_path = "src\\advent_of_code\\2023\\data\\day_9_test.txt";
    let file_path = "src\\advent_of_code\\2023\\data\\day_9.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        total += predict(input);
        //println!("hej total = {}", total);
    }
    
    println!("total = {}", total);
}


fn predict(mesures: Vec<i64>) -> i64 {
    let mut matrix = differences(mesures);
    let ml = matrix.len();
    matrix[ml-1].push(0);
    for i in (1..ml).rev() {
        let value = matrix[i-1][matrix[i].len()-1] + matrix[i][matrix[i].len()-1];
        matrix[i-1].push(value);
        //println!("matrix[i-1] = {:?}, i = {}, value = {:?}", matrix[i-1], i, value);
    }
    //println!("matix = {:?}, ml = {}, ml.rev = {:?}", matrix, ml, (1..ml).rev());
    matrix[0][matrix[0].len()-1]
}


fn differences(mesures: Vec<i64>) -> Vec<Vec<i64>> {
    let mut matrix = Vec::new();
    matrix.push(mesures);
    loop {
        if matrix[matrix.len()-1].iter().all(|&x| x == 0) {break;}
        matrix.push(Vec::new());
        let ml = matrix[matrix.len()-2].len()-1;
        for i in 0..ml {
            let value = matrix[matrix.len()-2][i+1] - matrix[matrix.len()-2][i];
            let ml2 = matrix.len()-1;
            matrix[ml2].push(value);
        }
    }
    matrix
}


fn fix_input(line: &str) -> Vec<i64>{
    let values = line.split(" ").map(|x| x.trim().parse::<i64>().unwrap()).collect();
    values
}
