use std::io;

fn main(){
    let mut total: i64 = 1;

    let mut times: Vec<i64> = read_input()
        .split(":").nth(1).unwrap().trim()
        .split(" ").filter(|x| x.len() > 0)
        .map(|x| x.parse::<i64>().unwrap()).collect();
    let mut records: Vec<i64> = read_input()
        .split(":").nth(1).unwrap().trim()
        .split(" ").filter(|x| x.len() > 0)
        .map(|x| x.parse::<i64>().unwrap()).collect();

    for _ in 0..times.len() {
        total *= beat_record(times.pop().unwrap(), records.pop().unwrap());
    }

    println!("total = {}", total);
}


fn beat_record(time: i64, record: i64) -> i64 {
    let mut beat_record = 0;
    for i in 0..time {
        if record < (time-i)*i {
            beat_record += 1;
        }
    }

    beat_record
}



fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("F");
    input.trim().to_string()
}