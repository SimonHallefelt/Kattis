use std::fs;

fn main(){
    let mut total: i64 = 0;

    //let file_path = "src\\advent_of_code\\2023\\data\\day_15_test.txt"; //145
    let file_path = "src\\advent_of_code\\2023\\data\\day_15.txt"; //265345
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");


    let mut map: Vec<Vec<(String, i64)>> = Vec::new();
    for i in 0..256 {
        map.push(Vec::new())
    }
    for c in contents.trim().split("\n") {
        let input = fix_input(c);
        println!("{}", input.len());
        for i in input {
            let temp = hash_word(i);
            let lable = temp.1;
            if temp.2 == 0 {
                let mut index = usize::MAX;
                for j in 0..map[temp.0 as usize].len() {
                    if map[temp.0 as usize][j].0 == lable {
                        index = j;
                        break;
                    }
                }
                if index != usize::MAX {
                    map[temp.0 as usize].remove(index);
                }
            } else {
                let mut index = usize::MAX;
                for j in 0..map[temp.0 as usize].len() {
                    if map[temp.0 as usize][j].0 == lable {
                        index = j;
                        break;
                    }
                }
                if index == usize::MAX {
                    map[temp.0 as usize].push((lable, temp.2));
                } else {
                    map[temp.0 as usize][index] = (lable, temp.2);
                }
            }
        }
    }

    println!("----");
    println!("map = {:?}", map);
    println!("----");

    total += calc_map(map);

    println!("total = {}", total);
}


fn calc_map(map: Vec<Vec<(String, i64)>>) -> i64 {
    let mut total = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let t = (i as i64 +1)*(j as i64 +1)*map[i][j].1;
            println!("t = {}, i = {}, j = {}, map[i][j].1 = {}", t, i, j, map[i][j].1);
            total += (i as i64 +1)*(j as i64 +1)*map[i][j].1;
        }
    }
    total
}


fn hash_word(word: Vec<char>) -> (i64, String, i64) {
    let mut total = 0;
    let mut lable = "".to_string();
    let mut value = 0;
    for temp in word.iter().enumerate() {
        let c = *temp.1;
        if c == '-' {break;}
        if c == '=' {value = word[temp.0 + 1].to_digit(10).unwrap() as i64; break;}
        lable.push(c);
        let u_value = c as u32;
        total += u_value as i64;
        total *= 17;
        total %= 256;
    }
    println!("total = {}, lable = {}, value = {}", total, lable, value);
    (total, lable, value)
}


fn fix_input(line: &str) -> Vec<Vec<char>> {
    let mut words = Vec::new();
    for w in line.trim().split(",") {
        words.push(w.trim().chars().collect())
    }
    words
}
