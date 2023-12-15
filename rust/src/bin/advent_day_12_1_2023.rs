use std::fs;

fn main(){
    let mut total: i64 = 0;
    
    //let file_path = "D:\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_12_test.txt"; //21
    let file_path = "D:\\Kattis\\rust\\src\\advent_of_code\\2023\\data\\day_12.txt"; //7753
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        let data = fix_data(input.0);
        let damages = input.1;

        //println!("{:?}", data);

        //total += damage_combinations(data, damages);
        total += dp(&data, &damages, 0, 0);
        print!("{} ", dp(&data, &damages, 0, 0));
    }
    println!("");

    println!("total = {}", total);
}


fn dp(data: &Vec<usize>, damages: &Vec<usize>, p: usize, d: usize) -> i64 {
    let mut combinations = 0;
    for i in p..data.len()+1-damages[d] {
        if data[i] == 1 {
            let mut b = fits(data, damages, i, d);
            if !b {break;}

            //får sättas in?
            if d+1 == damages.len() {
                for j in i+damages[d]..data.len() {
                    if data[j] == 1 {b = false; break;}
                }
                if !b {break;}
                combinations += 1;
            } else if i+damages[d] < data.len()+1-damages[d+1] {
                combinations += dp(data, damages, i+damages[d]+1, d+1);
            }
            break;
        } else if data[i] == 2 {
            let mut b = fits(data, damages, i, d);
            if !b {continue;}

            if d+1 == damages.len() {
                for j in i+damages[d]..data.len() {
                    if data[j] == 1 {b = false; break;}
                }
                if !b {continue;}
                combinations += 1;
            } else if i+damages[d] < data.len()+1-damages[d+1] {
                combinations += dp(data, damages, i+damages[d]+1, d+1);
            }
        }
    }

    combinations
}


fn fits(data: &Vec<usize>, damages: &Vec<usize>, i: usize, d: usize) -> bool {
    for j in i+1..i+damages[d] {
        if data[j] == 0 {return false}
    }
    if i+damages[d] < data.len() && data[i+damages[d]] == 1 {return false}
    true
}


fn fix_data(data: Vec<char>) -> Vec<usize> {
    let mut d = Vec::new();
    let mut last = false;
    for c in data {
        if !(c == '.' && last) {
            match c {
                '.' => {d.push(0); last = true},
                '#' => {d.push(1); last = false},
                _ => {d.push(2); last = false}
            }
        }
    }
    d
}


fn fix_input(line: &str) -> (Vec<char>, Vec<usize>) {
    let parts: Vec<&str> = line.trim().split(" ").collect();
    if parts.len() != 2 {
        println!("{:?}", parts);
    }
    (parts[0].trim().chars().collect(), parts[1].trim().split(",").map(|x| x.parse::<usize>().unwrap()).collect())
}


//tests
#[cfg(test)]
mod tests {
    //use std::time::SystemTime;

    use super::*;

    #[test]
    fn sample_input_1() {
        let data = Vec::from([1,0,1,0,1,1,1]);
        let damages = Vec::from([1,1,3]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn sample_input_2() {
        let data = Vec::from([1,0,1,0,1,1,2]);
        let damages = Vec::from([1,1,3]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn sample_input_3() {
        let data = Vec::from([1,0,1,0,1,2,2,0]);
        let damages = Vec::from([1,1,3]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn sample_input_4() {
        let data = Vec::from([1,0,1,0,2,2,2]);
        let damages = Vec::from([1,1,3]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn sample_input_5() {
        let data = Vec::from([1,0,1,0,2,2,1]);
        let damages = Vec::from([1,1,3]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn sample_input_6() {
        let data = Vec::from([1,0,1,0,2,2,1]);
        let damages = Vec::from([1,1,2]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn all_question_marks_() {
        let data = Vec::from([2,2,2,2,2,2]);
        let damages = Vec::from([1,1,1]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 4);
    }

    #[test]
    fn find_last_1() {
        let data = Vec::from([2,2,2,2,2,2,0,1]);
        let damages = Vec::from([1,1,1,1]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 4);
    }

    #[test]
    fn find_last_2() {
        let data = Vec::from([1,0,1,1,1,1,0,1]);
        let damages = Vec::from([1,4,1]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn find_last_3() {
        let data = Vec::from([2,0,2,2,0,2,0,2,2]);
        let damages = Vec::from([1,2,2]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }

    #[test]
    fn find_last_4() {
        let data = Vec::from([2,0,2,2,2,0,2,2,0,1,2]);
        let damages = Vec::from([1,3,2]);
        let answer = dp(&data, &damages, 0, 0);
        assert_eq!(answer, 1);
    }
}