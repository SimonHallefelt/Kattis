use std::io;

fn main(){
    let total: i64;

    let time: i64 = read_input()
        .split(":").nth(1).unwrap()
        .chars().filter(|x| x != &' ')
        .collect::<String>()
        .parse::<i64>().unwrap();
    let record: i64 = read_input()
        .split(":").nth(1).unwrap()
        .chars().filter(|x| x != &' ')
        .collect::<String>()
        .parse::<i64>().unwrap();

    total = beat_record(time, record);
    
    println!("total = {}", total); //38017587
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