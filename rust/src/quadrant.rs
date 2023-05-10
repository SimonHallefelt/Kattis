use std::io;

fn main(){
    let mut input1 = String::new();
    let mut input2 = String::new();

    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let input1 = input1.trim().to_string();
    let pos_x: i64 = input1.parse::<i64>().unwrap();

    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let input2 = input2.trim().to_string();
    let pos_y: i64 = input2.parse::<i64>().unwrap();

    if pos_x > 0 && pos_y > 0 {
        println!("1");
    }else if pos_x < 0 && pos_y > 0 {
        println!("2");
    }else if pos_x < 0 && pos_y < 0 {
        println!("3");
    }else{
        println!("4");
    }
}