use std::fs::read_to_string;

fn main() {
    let file_path = "src\\advent_of_code\\2024\\data\\day_9.txt"; // 6370402949053
    let (ids, file) = read_input(file_path);

    let mut tot = 0;
    let mut f = 0;
    let mut b = 1;
    let mut even = false;
    'o: for i in 0..file.len() {
        even = !even;
        for _ in 0..*file.get(i).unwrap() {
            if even {
                tot += (f + b - 1) as u64 * ids.get(f).unwrap();
                f += 1;
            } else {
                tot += (f + b - 1) as u64 * ids.get(ids.len()-b).unwrap();
                b += 1;
            }
            if f > ids.len()-b {
                break 'o;
            }
        }
    }

    println!("{}", tot);
}

fn read_input(path: &str) -> (Vec<u64>, Vec<u64>) {
    let content = read_to_string(path).expect("could not read file");
    let file: Vec<u64> = content.trim().chars()
        .map(|b| b.to_digit(10).unwrap() as u64).collect();
    let ids = file.iter()
        .enumerate()
        .filter(|a| a.0 % 2 == 0)
        .map(|b| vec![b.0 as u64 / 2; *b.1 as usize])
        .flatten()
        .collect();
    (ids, file)
}