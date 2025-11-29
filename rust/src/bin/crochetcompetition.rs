use std::{collections::HashMap, io};

fn main() {
    let m = HashMap::from([("Mon", 0),("Tue", 1),("Wed", 2),("Thu", 3),("Fri", 4),("Sat", 5),("Sun", 6)]);
    let mut input = read_line_stdin();
    let (start_day, start_hour, start_minute) = (*m.get(&input[0].as_str()).unwrap(), input[1].parse::<i32>().unwrap(), input[2].parse::<i32>().unwrap());
    input = read_line_stdin();
    let (end_day, end_hour, end_minute) = (*m.get(&input[0].as_str()).unwrap(), input[1].parse::<i32>().unwrap(), input[2].parse::<i32>().unwrap());
    if start_day == end_day && start_hour == end_hour && start_minute == end_minute {
        println!("7 days");
        return;
    }

    let start_time = start_day*24*60 + start_hour*60 + start_minute;
    let end_time = (end_day+7)*24*60 + end_hour*60 + end_minute;
    let diff = (end_time - start_time) % (7*24*60);

    let min = diff % 60;
    let hour = (diff / 60) % 24;
    let days = diff / (24 * 60);

    let mut times = Vec::new();
    if days == 1 {
        times.push("1 day".to_string());
    }
    else if days > 1 {
        times.push(days.to_string() + " days");
    }
    if hour == 1 {
        times.push("1 hour".to_string());
    }
    else if hour > 1 {
        times.push(hour.to_string() + " hours");
    }
    if min == 1 {
        times.push("1 minute".to_string());
    }
    else if min > 1 {
        times.push(min.to_string() + " minutes");
    }

    if times.len() == 1 {
        println!("{}", times[0])
    }
    if times.len() == 2 {
        println!("{} and {}", times[0], times[1])
    }
    if times.len() == 3 {
        println!("{}, {}, {}", times[0], times[1], times[2])
    }
}

fn read_line_stdin() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input = input.trim().to_string();
    let sp1 = input.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let sp2 = sp1[1].split(":").map(|s| s.to_string()).collect::<Vec<String>>();
    vec![sp1[0].clone(), sp2[0].clone(), sp2[1].clone()]
}