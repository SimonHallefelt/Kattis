use std::fs;

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_15_test.txt"; //1320
    let file_path = "src\\advent_of_code\\2023\\data\\day_15.txt"; //513643
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");



    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        println!("{}", input.len());
        for i in input {
            total += hash_word(i)
        }

    }


    println!("total = {}", total);
}


fn hash_word(word: Vec<char>) -> i64 {
    let mut total = 0;
    for c in word {
        let u_value = c as u32;
        total += u_value as i64;
        total *= 17;
        total %= 256;
    }
    total
}


fn fix_input(line: &str) -> Vec<Vec<char>> {
    let mut words = Vec::new();
    for w in line.trim().split(",") {
        words.push(w.trim().chars().collect())
    }
    words
}
