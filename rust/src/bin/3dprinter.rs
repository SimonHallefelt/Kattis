use std::io;

fn main(){
    let n = read_line().parse::<i128>().unwrap();
    let mut i = 0;
    loop{
        if 2_i128.pow(i as u32) >= n{
            println!("{}", i+1);
            break;
        }
        i += 1;
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}
