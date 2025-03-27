use std::fs::read_to_string;

fn main() {
    // let file_path = "src\\advent_of_code\\2024\\data\\day_9_test.txt"; // 2858
    let file_path = "src\\advent_of_code\\2024\\data\\day_9.txt"; // 6398096697992
    let (mut ids,mut openings) = read_input(file_path);

    let mut tot = 0;
    for (id, i) in ids.iter_mut().enumerate().rev() {
        for o in openings.iter_mut() {
            if i.1 < o.1 {
                break;
            } else if o.0 >= i.0 {
                i.1 = o.1;
                o.0 -= i.0;
                o.1 += i.0;
                break;
            } 
        }
        for p in 0..i.0 {
            tot += id as u64 * (i.1 + p);
        }
    }

    println!("{}", tot);
}

fn read_input(path: &str) -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
    let content = read_to_string(path).expect("could not read file");
    let file: Vec<u64> = content.trim().chars()
        .map(|b| b.to_digit(10).unwrap() as u64).collect();
    let mut ids = Vec::new();
    let mut openings = Vec::new();
    let mut sum = 0;
    for i in file.iter().enumerate() {
        if i.0 % 2 == 0 {
            ids.push((*i.1, sum));
        } else {
            openings.push((*i.1, sum));
        }
        sum += i.1;
    }

    (ids, openings)
}